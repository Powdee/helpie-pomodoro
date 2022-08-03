import { writable } from 'svelte/store';
import type { IPomodoro } from '../interface';

const pomodoro_store_map = new Map<number, IPomodoro>();
export const store_pomodoro =  writable(pomodoro_store_map);