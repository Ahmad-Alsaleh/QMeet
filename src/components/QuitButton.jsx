'use client';

const QuitButton = () => {
  return (
    <button
      onClick={() => async () => await exit(0)}
      className="w-full bg-gradient-to-r from-red-500 to-red-600 hover:from-red-600 hover:to-red-700 text-white font-semibold py-4 px-8 rounded-2xl transition-all duration-300 transform hover:scale-105 hover:shadow-lg shadow-red-200"
    >
      🚪 Quit App
    </button >
  );
};

export default QuitButton;