export interface Score {
  name: string;
  score: number;
  level: number;
  lines: number;
  timestamp: number;
}

const API_BASE = "/api";

export async function getHighScores(): Promise<Score[]> {
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

export async function submitScore(score: Omit<Score, "timestamp">): Promise<boolean> {
  try {
    const response = await fetch(`${API_BASE}/scores`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(score),
    });
    return response.ok;
  } catch (error) {
    console.error("Failed to submit score:", error);
    return false;
  }
}

export function formatScore(score: number): string {
  return score.toLocaleString();
}

export function formatDate(timestamp: number): string {
  return new Date(timestamp).toLocaleDateString();
}
