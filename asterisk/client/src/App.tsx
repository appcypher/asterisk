import type { Component } from 'solid-js';
import { createEffect, onMount } from 'solid-js';
import styles from './App.module.css';
import { autofocus } from '@solid-primitives/autofocus';
import { useKeyDownEvent } from "@solid-primitives/keyboard";
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';


//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const App: Component = () => {
  const event = useKeyDownEvent();
  let inputElementRef!: HTMLInputElement;

  return (
    <form data-tauri-drag-region>
      <input data-tauri-drag-region ref={inputElementRef} type="text" placeholder="Search" class={styles.input} />
    </form>
  );
};

export default App;
