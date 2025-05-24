'use client';
import { exit } from '@tauri-apps/plugin-process';
import ShortcutInput from '@/components/ShortcutInput';
import QuitButton from '@/components/QuitButton';

export default function Home() {
  return (
    <main className="min-h-screen bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 flex items-center justify-center p-5">
      <div className="bg-white/95 backdrop-blur-lg rounded-3xl p-10 shadow-2xl max-w-lg w-full text-center">
        <h1 className="text-4xl font-light text-gray-800 mb-8">
          ⌨️ QMeet
        </h1>
        <label className="block text-left text-gray-700 font-semibold mb-3 text-lg">
          Shortcut
        </label>
        <ShortcutInput />
        <QuitButton />
      </div>
    </main>
  )

}
