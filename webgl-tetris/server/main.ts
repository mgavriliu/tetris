/// <reference lib="deno.ns" />
/// <reference lib="deno.unstable" />

import { dirname, fromFileUrl, join } from "https://deno.land/std@0.208.0/path/mod.ts";

const __dirname = dirname(fromFileUrl(import.meta.url));
const rootDir = join(__dirname, "..");

interface Score {
  name: string;
  score: number;
  level: number;
  lines: number;
  timestamp: number;
}

// Use Deno KV for persistent storage
const kv = await Deno.openKv();

async function loadScores(): Promise<Score[]> {
  const scores: Score[] = [];
  const entries = kv.list<Score>({ prefix: ["scores"] });
  for await (const entry of entries) {
    scores.push(entry.value);
  }
  return scores;
}

async function saveScore(score: Score): Promise<void> {
  await kv.set(["scores", score.timestamp], score);

  const scores = await loadScores();
  scores.sort((a, b) => b.score - a.score);
  if (scores.length > 100) {
    for (const s of scores.slice(100)) {
      await kv.delete(["scores", s.timestamp]);
    }
  }
}

function isValidScore(obj: unknown): obj is Omit<Score, "timestamp"> {
  if (typeof obj !== "object" || obj === null) return false;
  const s = obj as Record<string, unknown>;
  return (
    typeof s.name === "string" &&
    s.name.length > 0 &&
    s.name.length <= 20 &&
    typeof s.score === "number" &&
    s.score >= 0 &&
    typeof s.level === "number" &&
    s.level >= 1 &&
    typeof s.lines === "number" &&
    s.lines >= 0
  );
}

const MIME_TYPES: Record<string, string> = {
  ".html": "text/html",
  ".js": "application/javascript",
  ".wasm": "application/wasm",
  ".css": "text/css",
  ".json": "application/json",
};

async function handler(req: Request): Promise<Response> {
  const url = new URL(req.url);
  const path = url.pathname;

  console.log(`${req.method} ${path}`);

  // Health check
  if (path === "/health") {
    return Response.json({ status: "ok" });
  }

  // API: Get scores
  if (path === "/api/scores" && req.method === "GET") {
    const scores = await loadScores();
    scores.sort((a, b) => b.score - a.score);
    return Response.json(scores.slice(0, 10));
  }

  // API: Submit score
  if (path === "/api/scores" && req.method === "POST") {
    try {
      const body = await req.json();
      if (!isValidScore(body)) {
        return Response.json({ error: "Invalid score data" }, { status: 400 });
      }
      const score: Score = {
        name: body.name.trim().substring(0, 20),
        score: Math.floor(body.score),
        level: Math.floor(body.level),
        lines: Math.floor(body.lines),
        timestamp: Date.now(),
      };
      await saveScore(score);
      return Response.json({ success: true }, { status: 201 });
    } catch (e) {
      console.error("Error submitting score:", e);
      return Response.json({ error: "Internal server error" }, { status: 500 });
    }
  }

  // Static files
  try {
    let filePath: string;

    if (path === "/" || path === "/index.html") {
      filePath = join(rootDir, "frontend/index.html");
    } else if (path.startsWith("/pkg/")) {
      filePath = join(rootDir, path.slice(1));
    } else if (path.startsWith("/dist/")) {
      filePath = join(rootDir, "frontend", path);
    } else {
      filePath = join(rootDir, "frontend", path);
    }

    const file = await Deno.readFile(filePath);
    const ext = filePath.substring(filePath.lastIndexOf("."));
    const contentType = MIME_TYPES[ext] || "application/octet-stream";

    return new Response(file, {
      headers: {
        "Content-Type": contentType,
        "X-Frame-Options": "ALLOWALL",
      },
    });
  } catch (e) {
    console.error(`Error serving ${path}:`, e);
    return new Response("Not Found", { status: 404 });
  }
}

const port = parseInt(Deno.env.get("PORT") ?? "8000");
console.log(`Tetris server running on http://localhost:${port}`);

Deno.serve({ port }, handler);
