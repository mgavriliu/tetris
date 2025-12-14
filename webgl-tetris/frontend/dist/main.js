// frontend/main.ts
import init, { TetrisApp } from "../pkg/tetris_core.js";

// frontend/api.ts
var API_BASE = "/api";
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
var STATE_IDLE = 0;
var STATE_PLAYING = 1;
var STATE_PAUSED = 2;
var STATE_GAME_OVER = 3;
var CELL_SIZE = 30;
var BOARD_WIDTH = 10;
var BOARD_HEIGHT = 20;
var PREVIEW_CELL_SIZE = CELL_SIZE * 0.8;
var scoreEl;
var levelEl;
var linesEl;
var scoresEl;
var gameOverOverlay;
var app;
async function main() {
  try {
    console.log("Initializing WASM...");
    await init("/pkg/tetris_core_bg.wasm");
    console.log("Creating UI...");
    const { boardCanvas, nextCanvas, holdCanvas } = createUI();
    console.log("Creating TetrisApp...");
    app = new TetrisApp(boardCanvas, nextCanvas, holdCanvas);
    console.log("Setting up callbacks...");
    setupCallbacks();
    console.log("Setting up input handlers...");
    setupInputHandlers();
    console.log("Loading high scores...");
    await loadHighScores();
    console.log("Showing start screen...");
    showStartScreen();
    console.log("Starting game loop...");
    app.render_initial();
    app.start_game_loop();
    console.log("Initialization complete!");
  } catch (e) {
    console.error("Initialization error:", e);
    document.body.innerHTML = `<pre style="color:red;padding:20px;">Error: ${e}
${e.stack}</pre>`;
  }
}
function createUI() {
  const container = document.getElementById("game-container");
  const boardCanvas = document.createElement("canvas");
  boardCanvas.width = BOARD_WIDTH * CELL_SIZE + 6;
  boardCanvas.height = BOARD_HEIGHT * CELL_SIZE + 2;
  boardCanvas.className = "tetris-board";
  const nextCanvas = document.createElement("canvas");
  nextCanvas.width = 4 * PREVIEW_CELL_SIZE;
  nextCanvas.height = 2 * PREVIEW_CELL_SIZE;
  nextCanvas.className = "preview-canvas";
  const holdCanvas = document.createElement("canvas");
  holdCanvas.width = 4 * PREVIEW_CELL_SIZE;
  holdCanvas.height = 2 * PREVIEW_CELL_SIZE;
  holdCanvas.className = "preview-canvas";
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
  const boardWrapper = document.createElement("div");
  boardWrapper.className = "board-wrapper";
  boardWrapper.appendChild(boardCanvas);
  gameOverOverlay = document.createElement("div");
  gameOverOverlay.className = "game-over-overlay";
  gameOverOverlay.innerHTML = `
    <div class="game-over-text">GAME OVER</div>
    <div class="game-over-subtext">Press R to restart</div>
  `;
  gameOverOverlay.style.display = "none";
  boardWrapper.appendChild(gameOverOverlay);
  gameArea.appendChild(leftPanel);
  gameArea.appendChild(boardWrapper);
  gameArea.appendChild(rightPanel);
  container.appendChild(gameArea);
  document.getElementById("hold-container").appendChild(holdCanvas);
  document.getElementById("next-container").appendChild(nextCanvas);
  scoreEl = document.getElementById("score");
  levelEl = document.getElementById("level");
  linesEl = document.getElementById("lines");
  scoresEl = document.getElementById("ingame-scores");
  return { boardCanvas, nextCanvas, holdCanvas };
}
function setupCallbacks() {
  app.set_on_state_change((newState, oldState) => {
    if (newState === STATE_GAME_OVER) {
      showGameOver();
      handleGameOver();
    } else if (newState === STATE_PAUSED) {
      showPauseOverlay();
    } else if (newState === STATE_PLAYING && oldState !== STATE_PLAYING) {
      clearOverlay();
    } else if (newState === STATE_IDLE) {
      showStartScreen();
    }
  });
  app.set_on_score_update((score, level, lines) => {
    scoreEl.textContent = String(score);
    levelEl.textContent = String(level);
    linesEl.textContent = String(lines);
  });
}
function setupInputHandlers() {
  const gameKeys = [
    "ArrowLeft",
    "ArrowRight",
    "ArrowDown",
    "ArrowUp",
    "Space",
    "KeyX",
    "KeyZ",
    "KeyC",
    "KeyP",
    "Escape",
    "Enter",
    "KeyR",
    "ShiftLeft",
    "ShiftRight",
    "ControlLeft",
    "ControlRight"
  ];
  window.addEventListener("keydown", (e) => {
    if (gameKeys.includes(e.code)) {
      e.preventDefault();
      app.key_down(e.code);
    }
  });
  window.addEventListener("keyup", (e) => {
    app.key_up(e.code);
  });
  window.addEventListener("blur", () => {
    app.on_blur();
  });
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
  gameOverOverlay.style.display = "flex";
}
function clearOverlay() {
  if (gameOverOverlay) gameOverOverlay.style.display = "none";
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
  const score = app.get_score();
  const level = app.get_level();
  const lines = app.get_lines();
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
