// frontend/main.ts
import init, { Tetris, get_color } from "../../pkg/tetris_core.js";

// frontend/api.ts
var API_BASE = "http://localhost:8000/api";
async function getHighScores() {
  try {
    const response = await fetch(`${API_BASE}/scores`);
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error("Failed to fetch high scores:", error);
    return [];
  }
}
async function submitScore(score) {
  try {
    const response = await fetch(`${API_BASE}/scores`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(score)
    });
    return response.ok;
  } catch (error) {
    console.error("Failed to submit score:", error);
    return false;
  }
}

// frontend/main.ts
var CELL_SIZE = 30;
var BOARD_WIDTH = 10;
var BOARD_HEIGHT = 20;
var KEY_MAP = {
  ArrowLeft: 0,
  ArrowRight: 1,
  ArrowDown: 2,
  Space: 3,
  ArrowUp: 4,
  KeyX: 4,
  KeyZ: 5,
  ControlLeft: 5,
  ControlRight: 5,
  KeyC: 6,
  ShiftLeft: 6,
  ShiftRight: 6,
  KeyP: 7,
  Escape: 7,
  Enter: 8,
  KeyR: 9
};
var STATE_IDLE = 0;
var STATE_PLAYING = 1;
var STATE_PAUSED = 2;
var STATE_GAME_OVER = 3;
var svg;
var boardGroup;
var pieceGroup;
var ghostGroup;
var overlayGroup;
var nextSvg;
var holdSvg;
var scoreEl;
var levelEl;
var linesEl;
var scoresEl;
var tetris;
var lastTime = 0;
var lastState = STATE_IDLE;
async function main() {
  await init("../pkg/tetris_core_bg.wasm");
  tetris = new Tetris();
  createUI();
  setupInputHandlers();
  await loadHighScores();
  showStartScreen();
  requestAnimationFrame(gameLoop);
}
function createUI() {
  const container = document.getElementById("game-container");
  svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  svg.setAttribute("width", String(BOARD_WIDTH * CELL_SIZE + 6));
  svg.setAttribute("height", String(BOARD_HEIGHT * CELL_SIZE + 2));
  svg.setAttribute("class", "tetris-board");
  boardGroup = document.createElementNS("http://www.w3.org/2000/svg", "g");
  ghostGroup = document.createElementNS("http://www.w3.org/2000/svg", "g");
  pieceGroup = document.createElementNS("http://www.w3.org/2000/svg", "g");
  overlayGroup = document.createElementNS("http://www.w3.org/2000/svg", "g");
  svg.appendChild(boardGroup);
  svg.appendChild(ghostGroup);
  svg.appendChild(pieceGroup);
  svg.appendChild(overlayGroup);
  for (let y = 0; y < BOARD_HEIGHT; y++) {
    for (let x = 0; x < BOARD_WIDTH; x++) {
      const rect = document.createElementNS("http://www.w3.org/2000/svg", "rect");
      rect.setAttribute("x", String(x * CELL_SIZE));
      rect.setAttribute("y", String(y * CELL_SIZE));
      rect.setAttribute("width", String(CELL_SIZE));
      rect.setAttribute("height", String(CELL_SIZE));
      rect.setAttribute("fill", get_color(0));
      rect.setAttribute("stroke", "#2a2a4a");
      rect.setAttribute("stroke-width", "1");
      boardGroup.appendChild(rect);
    }
  }
  nextSvg = createPreviewSvg();
  holdSvg = createPreviewSvg();
  const gameArea = document.createElement("div");
  gameArea.className = "game-area";
  const leftPanel = document.createElement("div");
  leftPanel.className = "side-panel";
  leftPanel.innerHTML = `
    <div class="panel-section">
      <div class="panel-label">HOLD</div>
      <div class="preview-container" id="hold-container"></div>
    </div>
    <div class="panel-section">
      <div class="panel-label">HIGH SCORES</div>
      <div id="ingame-scores" class="ingame-scores"></div>
    </div>
  `;
  const rightPanel = document.createElement("div");
  rightPanel.className = "side-panel";
  rightPanel.innerHTML = `
    <div class="panel-section">
      <div class="panel-label">NEXT</div>
      <div class="preview-container" id="next-container"></div>
    </div>
    <div class="panel-section">
      <div class="panel-label">SCORE</div>
      <div class="stat-value" id="score">0</div>
    </div>
    <div class="panel-section">
      <div class="panel-label">LEVEL</div>
      <div class="stat-value" id="level">1</div>
    </div>
    <div class="panel-section">
      <div class="panel-label">LINES</div>
      <div class="stat-value" id="lines">0</div>
    </div>
  `;
  gameArea.appendChild(leftPanel);
  gameArea.appendChild(svg);
  gameArea.appendChild(rightPanel);
  container.appendChild(gameArea);
  document.getElementById("hold-container").appendChild(holdSvg);
  document.getElementById("next-container").appendChild(nextSvg);
  scoreEl = document.getElementById("score");
  levelEl = document.getElementById("level");
  linesEl = document.getElementById("lines");
  scoresEl = document.getElementById("ingame-scores");
}
function createPreviewSvg() {
  const s = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  s.setAttribute("width", String(4 * CELL_SIZE * 0.8));
  s.setAttribute("height", String(2 * CELL_SIZE * 0.8));
  s.setAttribute("class", "preview-svg");
  return s;
}
function setupInputHandlers() {
  window.addEventListener("keydown", (e) => {
    const key = KEY_MAP[e.code];
    if (key !== void 0) {
      e.preventDefault();
      tetris.key_down(key);
    }
  });
  window.addEventListener("keyup", (e) => {
    const key = KEY_MAP[e.code];
    if (key !== void 0) {
      tetris.key_up(key);
    }
  });
  window.addEventListener("blur", () => {
    tetris.on_blur();
  });
}
function gameLoop(currentTime) {
  const deltaMs = lastTime ? currentTime - lastTime : 0;
  lastTime = currentTime;
  tetris.update(deltaMs);
  const state = tetris.get_state();
  if (state !== lastState) {
    if (state === STATE_GAME_OVER) {
      showGameOver();
      handleGameOver();
    } else if (state === STATE_PAUSED) {
      showPauseOverlay();
    } else if (state === STATE_PLAYING && lastState !== STATE_PLAYING) {
      clearOverlay();
    } else if (state === STATE_IDLE) {
      showStartScreen();
    }
    lastState = state;
  }
  if (state === STATE_PLAYING) {
    render();
  }
  requestAnimationFrame(gameLoop);
}
function render() {
  scoreEl.textContent = String(tetris.get_score());
  levelEl.textContent = String(tetris.get_level());
  linesEl.textContent = String(tetris.get_lines());
  ghostGroup.innerHTML = "";
  pieceGroup.innerHTML = "";
  const boardRects = boardGroup.querySelectorAll("rect");
  boardRects.forEach((rect) => rect.setAttribute("fill", get_color(0)));
  const boardCells = tetris.get_board_cells();
  for (let i = 0; i < boardCells.length; i += 4) {
    const x = boardCells[i];
    const y = boardCells[i + 1];
    const color = boardCells[i + 2];
    const idx = y * BOARD_WIDTH + x;
    boardRects[idx]?.setAttribute("fill", get_color(color));
  }
  const ghostCells = tetris.get_ghost_cells();
  renderCells(ghostGroup, ghostCells, CELL_SIZE);
  const pieceCells = tetris.get_piece_cells();
  renderCells(pieceGroup, pieceCells, CELL_SIZE);
  renderPreview(nextSvg, tetris.get_next_cells());
  renderPreview(holdSvg, tetris.get_hold_cells());
}
function renderCells(group, cells, cellSize) {
  for (let i = 0; i < cells.length; i += 4) {
    const x = cells[i];
    const y = cells[i + 1];
    const color = cells[i + 2];
    const opacity = cells[i + 3] / 255;
    const rect = document.createElementNS("http://www.w3.org/2000/svg", "rect");
    rect.setAttribute("x", String(x * cellSize + 1));
    rect.setAttribute("y", String(y * cellSize + 1));
    rect.setAttribute("width", String(cellSize - 2));
    rect.setAttribute("height", String(cellSize - 2));
    rect.setAttribute("fill", get_color(color));
    rect.setAttribute("opacity", String(opacity));
    rect.setAttribute("rx", "3");
    group.appendChild(rect);
  }
}
function renderPreview(svg2, cells) {
  svg2.innerHTML = "";
  const cellSize = CELL_SIZE * 0.8;
  for (let i = 0; i < cells.length; i += 4) {
    const x = cells[i];
    const y = cells[i + 1];
    const color = cells[i + 2];
    const opacity = cells[i + 3] / 255;
    const rect = document.createElementNS("http://www.w3.org/2000/svg", "rect");
    rect.setAttribute("x", String(x * cellSize + 1));
    rect.setAttribute("y", String(y * cellSize + 1));
    rect.setAttribute("width", String(cellSize - 2));
    rect.setAttribute("height", String(cellSize - 2));
    rect.setAttribute("fill", get_color(color));
    rect.setAttribute("opacity", String(opacity));
    rect.setAttribute("rx", "2");
    svg2.appendChild(rect);
  }
}
function showStartScreen() {
  clearOverlay();
  let screen = document.getElementById("start-screen");
  if (!screen) {
    screen = document.createElement("div");
    screen.id = "start-screen";
    screen.innerHTML = `
      <h1>TETRIS</h1>
      <p class="subtitle">Press SPACE or ENTER to start</p>
      <div class="controls">
        <h3>Controls</h3>
        <ul>
          <li><kbd>&larr;</kbd> <kbd>&rarr;</kbd> Move</li>
          <li><kbd>&darr;</kbd> Soft drop</li>
          <li><kbd>Space</kbd> Hard drop</li>
          <li><kbd>&uarr;</kbd> <kbd>X</kbd> Rotate CW</li>
          <li><kbd>Z</kbd> Rotate CCW</li>
          <li><kbd>C</kbd> <kbd>Shift</kbd> Hold</li>
          <li><kbd>P</kbd> <kbd>Esc</kbd> Pause</li>
        </ul>
      </div>
      <div id="high-scores"></div>
    `;
    document.getElementById("game-container")?.appendChild(screen);
    loadHighScores();
  }
  screen.style.display = "flex";
}
function showPauseOverlay() {
  clearOverlay();
  let overlay = document.getElementById("pause-overlay");
  if (!overlay) {
    overlay = document.createElement("div");
    overlay.id = "pause-overlay";
    overlay.innerHTML = `
      <h2>PAUSED</h2>
      <p>Press P or ESC to resume</p>
      <p>Press R to restart</p>
    `;
    document.getElementById("game-container")?.appendChild(overlay);
  }
  overlay.style.display = "flex";
}
function showGameOver() {
  const rect = document.createElementNS("http://www.w3.org/2000/svg", "rect");
  rect.setAttribute("width", String(BOARD_WIDTH * CELL_SIZE));
  rect.setAttribute("height", String(BOARD_HEIGHT * CELL_SIZE));
  rect.setAttribute("fill", "rgba(0,0,0,0.7)");
  overlayGroup.appendChild(rect);
  const text = document.createElementNS("http://www.w3.org/2000/svg", "text");
  text.setAttribute("x", String(BOARD_WIDTH * CELL_SIZE / 2));
  text.setAttribute("y", String(BOARD_HEIGHT * CELL_SIZE / 2));
  text.setAttribute("text-anchor", "middle");
  text.setAttribute("fill", "#fff");
  text.setAttribute("font-size", "24");
  text.setAttribute("font-family", "monospace");
  text.textContent = "GAME OVER";
  overlayGroup.appendChild(text);
  const subtext = document.createElementNS("http://www.w3.org/2000/svg", "text");
  subtext.setAttribute("x", String(BOARD_WIDTH * CELL_SIZE / 2));
  subtext.setAttribute("y", String(BOARD_HEIGHT * CELL_SIZE / 2 + 30));
  subtext.setAttribute("text-anchor", "middle");
  subtext.setAttribute("fill", "#888");
  subtext.setAttribute("font-size", "14");
  subtext.setAttribute("font-family", "monospace");
  subtext.textContent = "Press R to restart";
  overlayGroup.appendChild(subtext);
}
function clearOverlay() {
  overlayGroup.innerHTML = "";
  document.getElementById("start-screen")?.style.setProperty("display", "none");
  document.getElementById("pause-overlay")?.style.setProperty("display", "none");
}
async function loadHighScores() {
  const scores = await getHighScores();
  displayHighScores(scores);
  displayInGameScores(scores);
}
function displayHighScores(scores) {
  const container = document.getElementById("high-scores");
  if (!container) return;
  if (scores.length === 0) {
    container.innerHTML = "<p class='no-scores'>No high scores yet!</p>";
    return;
  }
  container.innerHTML = `
    <h3>High Scores</h3>
    <ol class="scores-list">
      ${scores.slice(0, 10).map((s) => `
        <li><span class="score-name">${escapeHtml(s.name)}</span>
        <span class="score-value">${s.score.toLocaleString()}</span></li>
      `).join("")}
    </ol>
  `;
}
function displayInGameScores(scores) {
  if (!scoresEl) return;
  if (scores.length === 0) {
    scoresEl.innerHTML = '<div class="no-scores">-</div>';
    return;
  }
  scoresEl.innerHTML = scores.slice(0, 5).map(
    (s, i) => `<div class="score-row"><span class="rank">${i + 1}.</span><span class="name">${escapeHtml(s.name)}</span><span class="pts">${s.score.toLocaleString()}</span></div>`
  ).join("");
}
function escapeHtml(text) {
  const div = document.createElement("div");
  div.textContent = text;
  return div.innerHTML;
}
async function handleGameOver() {
  const score = tetris.get_score();
  const level = tetris.get_level();
  const lines = tetris.get_lines();
  const scores = await getHighScores();
  const isHighScore = scores.length < 10 || score > (scores[9]?.score ?? 0);
  if (isHighScore && score > 0) {
    const name = prompt(`High Score! Enter your name (Score: ${score.toLocaleString()})`);
    if (name?.trim()) {
      await submitScore({ name: name.trim().substring(0, 20), score, level, lines });
      await loadHighScores();
    }
  }
}
main().catch(console.error);
