'use client';
import { exit } from '@tauri-apps/plugin-process';

const QuitButton = () => {
  return (
    <button
      onClick={async () => await exit(0)}
      className="cursor-pointer w-full sm:w-auto rounded-xl bg-red-500 px-6 py-3 text-white shadow hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-red-400 transition"
    >
      🚪 Quit App
    </button >
  );
};

export default QuitButton;