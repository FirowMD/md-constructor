import { writable } from 'svelte/store';
import type { MarkdownElement } from './types';

const defaultText = `Welcome to **MD Constructor**

---

Here is a simple markdown editor.
`;

export const elements = writable<MarkdownElement[]>([]);
export const rawText = writable<string>(defaultText);
export const compiledText = writable<string>(''); 