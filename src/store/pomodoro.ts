import { writable } from 'svelte/store';
import type { IPomodoro } from '../interface';

export default writable(new Map<number, IPomodoro>());