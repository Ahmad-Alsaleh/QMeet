import { invoke } from '@tauri-apps/api/core';

const QuitButton = () => {
  return (
    <button
      onClick={async () => await invoke("quit_app")}
      className="cursor-pointer w-full sm:w-auto rounded-xl bg-red-500 px-6 py-3 text-white shadow hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-red-400 transition"
    >
      🚪 Quit App
    </button >
  );
};

export default QuitButton;
