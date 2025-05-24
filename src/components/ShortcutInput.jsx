'use client';

import { useState, useRef } from 'react';

const ShortcutInput = () => {
  const [currentShortcut, setCurrentShortcut] = useState('');
  const [isRecording, setIsRecording] = useState(false);
  const inputRef = useRef(null);

  const getKeyName = (key, code) => {
    const specialKeys = {
      ' ': 'Space',
      'Enter': 'Enter',
      'Escape': 'Esc',
      'Tab': 'Tab',
      'Backspace': 'Backspace',
      'Delete': 'Delete',
      'Insert': 'Insert',
      'Home': 'Home',
      'End': 'End',
      'PageUp': 'PageUp',
      'PageDown': 'PageDown',
      'ArrowUp': 'Up',
      'ArrowDown': 'Down',
      'ArrowLeft': 'Left',
      'ArrowRight': 'Right',
      'CapsLock': 'CapsLock',
      'NumLock': 'NumLock',
      'ScrollLock': 'ScrollLock',
      'PrintScreen': 'PrintScreen',
      'Pause': 'Pause'
    };

    if (key.startsWith('F') && key.length <= 3) {
      return key;
    }

    if (code && code.startsWith('Numpad')) {
      return code.replace('Numpad', 'Num');
    }

    if (specialKeys[key]) {
      return specialKeys[key];
    }

    if (code && code.startsWith('Key')) {
      return code.replace('Key', '');
    }

    if (code && code.startsWith('Digit')) {
      return code.replace('Digit', '');
    }

    const codeToKey = {
      'Semicolon': ';',
      'Equal': '=',
      'Comma': ',',
      'Minus': '-',
      'Period': '.',
      'Slash': '/',
      'Backquote': '`',
      'BracketLeft': '[',
      'Backslash': '\\',
      'BracketRight': ']',
      'Quote': "'",
    };

    if (code && codeToKey[code]) {
      return codeToKey[code];
    }

    if (key.length === 1) {
      return key.toUpperCase();
    }

    return key;
  };

  const handleKeyDown = (e) => {
    e.preventDefault();

    if (!isRecording) return;

    const keys = [];

    // Add modifier keys
    if (e.ctrlKey) keys.push('Ctrl');
    if (e.altKey) keys.push('Alt');
    if (e.shiftKey) keys.push('Shift');
    if (e.metaKey) keys.push('Meta');

    // add the main key (if it's not a modifier)
    const mainKey = getKeyName(e.key, e.code);
    if (mainKey && !['Control', 'Alt', 'Shift', 'Meta'].includes(mainKey)) {
      keys.push(mainKey);
    }

    if (keys.length > 0) {
      const shortcut = keys.join('+');
      setCurrentShortcut(shortcut);
    }
  };

  const handleFocus = () => {
    setIsRecording(true);
  };

  const handleBlur = () => {
    setIsRecording(false);
  };

  const handleContextMenu = (e) => {
    e.preventDefault();
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 flex items-center justify-center p-5">
      <div className="bg-white/95 backdrop-blur-lg rounded-3xl p-10 shadow-2xl max-w-lg w-full text-center">
        <h1 className="text-4xl font-light text-gray-800 mb-8">
          ⌨️ QMeet
        </h1>

        <div className="relative mb-8">
          <input
            ref={inputRef}
            type="text"
            value={currentShortcut}
            onKeyDown={handleKeyDown}
            onFocus={handleFocus}
            onBlur={handleBlur}
            onContextMenu={handleContextMenu}
            placeholder={isRecording ? "Press your shortcut keys..." : "Click here and press your shortcut keys..."}
            className={`w-full p-5 text-lg border-3 rounded-2xl outline-none transition-all duration-300 text-center font-medium text-gray-800 ${isRecording
              ? 'border-red-400 bg-red-50 animate-pulse shadow-lg shadow-red-200'
              : 'border-gray-300 bg-gray-50 focus:border-indigo-400 focus:bg-white focus:shadow-lg focus:shadow-indigo-200'
              }`}
            readOnly
          />
          <div className="text-gray-600 text-sm mt-3 leading-relaxed">
            Click the input field and press any combination of keys.
          </div>
        </div>

      </div>
    </div>
  );
};

export default ShortcutInput;
