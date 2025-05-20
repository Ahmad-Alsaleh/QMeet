export default function Home() {
  return (
    <main className="flex min-h-screen items-center justify-center px-4 py-8">
      <div className="backdrop-blur-xl bg-white/70 border border-neutral-200 shadow-xl rounded-3xl px-8 py-10 w-full max-w-md text-center space-y-8">
        <h1 className="text-3xl md:text-4xl font-semibold text-neutral-900">
          QMeet Control
        </h1>
        <div className="flex flex-col sm:flex-row justify-center gap-4">
          <button className="w-full sm:w-auto rounded-xl bg-blue-600 px-6 py-3 text-white shadow hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-400 transition">
            Modify Shortcut
          </button>
          <button className="w-full sm:w-auto rounded-xl bg-red-500 px-6 py-3 text-white shadow hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-red-400 transition">
            Quit App
          </button>
        </div>
      </div>
    </main>
  );
}
