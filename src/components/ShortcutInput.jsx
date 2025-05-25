'use client';

import { useState, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';

const ShortcutInput = () => {
  const [currentShortcut, setCurrentShortcut] = useState('Ctrl+Alt+P');
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

  const handleKeyDown = async (e) => {
    e.preventDefault();

    if (!isRecording) return;

    const keys = [];

    if (e.ctrlKey) keys.push('Ctrl');
    if (e.altKey) keys.push('Alt');
    if (e.shiftKey) keys.push('Shift');
    if (e.metaKey) keys.push('Meta');

    const mainKey = getKeyName(e.key, e.code);
    if (mainKey && !['Control', 'Alt', 'Shift', 'Meta'].includes(mainKey)) {
      keys.push(mainKey);
    }

    if (keys.length > 0) {
      const shortcut = keys.join('+');
      setCurrentShortcut(shortcut);

      // Update the backend with the new shortcut
      try {
        await invoke('update_target_shortcut', { shortcutStr: shortcut });
        console.log('Shortcut updated successfully:', shortcut);
      } catch (error) {
        // TODO: show an error message to the user in the frontend
        // and return to the previous shortcut
        // TODO: treat ESC differently 
        console.error('Failed to update shortcut:', error);
      }
    }
  };

  const handleFocus = async () => {
    setIsRecording(true);
    await invoke('unregister_target_shortcut');
  };

  const handleBlur = async () => {
    setIsRecording(false);
  };

  const handleContextMenu = (e) => {
    e.preventDefault();
  };

  return (
    <div className="relative mb-8">
      <input
        ref={inputRef}
        type="text"
        value={currentShortcut}
        onKeyDown={handleKeyDown}
        onFocus={handleFocus}
        onBlur={handleBlur}
        onContextMenu={handleContextMenu}
        className={`w-full p-5 text-lg border-3 rounded-2xl outline-none transition-all duration-300 text-center font-medium text-gray-800 ${isRecording
          ? 'border-blue-400 bg-blue-50 shadow-lg shadow-blue-200'
          : 'border-gray-300 bg-gray-50 focus:border-indigo-400 focus:bg-white focus:shadow-lg focus:shadow-indigo-200'
          }`}
        readOnly
      />
      <div className="text-gray-600 text-sm mt-3 leading-relaxed">
        Click the input field and press any combination of keys.
      </div>
    </div>
  );
};

export default ShortcutInput;
