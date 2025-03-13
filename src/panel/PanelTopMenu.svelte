<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Tabs } from '@skeletonlabs/skeleton-svelte';
  import { elements, rawText, compiledText } from '../lib/stores';
  import type { MarkdownConverter, MarkdownElement } from '../lib/types';
  import { open, save, message } from '@tauri-apps/plugin-dialog';

  let showFileMenu = $state(false);

  // Auto-compile when text or elements change
  $effect(() => {
    const converter: MarkdownConverter = {
      raw_text: $rawText,
      elements: $elements
    };
    
    invoke('update_markdown_converter', { markdownConverter: converter })
      .then(result => compiledText.set(result as string))
      .catch(err => console.error("Compilation error:", err));
  });

  async function handleOpen() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'JSON',
          extensions: ['json']
        }]
      });
      
      if (selected) {
        const content = await invoke('read_json_file', { path: selected });
        if (content && typeof content === 'object') {
          const data = content as any;
          if (data.raw_text && Array.isArray(data.elements)) {
            rawText.set(data.raw_text);
            
            // Get expanded elements before updating elements store
            const expandedElements = data.elements
              .filter((element: MarkdownElement) => !element.is_collapsed)
              .map((element: MarkdownElement) => element.element_name);
            
            elements.set(data.elements);
            
            // Update value array in PanelSide
            const event = new CustomEvent('updateAccordion', { 
              detail: { value: expandedElements } 
            });
            window.dispatchEvent(event);
          }
        }
      }
    } catch (err) {
      console.error("Error opening file:", err);
    }
    showFileMenu = false;
  }

  async function handleSave() {
    try {
      const filePath = await save({
        filters: [{
          name: 'JSON',
          extensions: ['json']
        }]
      });
      
      if (filePath) {
        const data = {
          raw_text: $rawText,
          elements: $elements
        };
        await invoke('write_json_file', { 
          path: filePath,
          content: data
        });
      }
    } catch (err) {
      console.error("Error saving file:", err);
    }
    showFileMenu = false;
  }

  async function handleExport() {
    try {
      const filePath = await save({
        filters: [{
          name: 'Markdown',
          extensions: ['md']
        }]
      });
      
      if (filePath) {
        await invoke('write_text_file', { 
          path: filePath,
          content: $compiledText
        });
      }
    } catch (err) {
      console.error("Error exporting file:", err);
    }
    showFileMenu = false;
  }

  async function handleAbout() {
    await message('MD Constructor v0.1.0 (c) FirowMD', { title: 'About' });
  }

  function toggleFileMenu() {
    showFileMenu = !showFileMenu;
  }

  // Close menu when clicking outside
  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target.closest('.file-menu-container')) {
      showFileMenu = false;
    }
  }
</script>

<svelte:window onclick={handleClickOutside} />

<div class="flex flex-row w-full h-10 bg-surface-900 text-base justify-between">
  <div class="flex flex-row">
    <div class="relative file-menu-container">
      <button 
        type="button" 
        class="btn preset-filled-surface rounded-none" 
        onclick={toggleFileMenu}
      >
        File
      </button>
      
      {#if showFileMenu}
        <div class="absolute top-10 left-0 bg-surface-800 border border-surface-700 rounded shadow-lg z-50">
          <button 
            class="w-full px-4 py-2 text-left hover:bg-surface-700 transition-colors"
            onclick={handleOpen}
          >
            Open
          </button>
          <button 
            class="w-full px-4 py-2 text-left hover:bg-surface-700 transition-colors"
            onclick={handleSave}
          >
            Save
          </button>
          <button 
            class="w-full px-4 py-2 text-left hover:bg-surface-700 transition-colors"
            onclick={handleExport}
          >
            Export
          </button>
        </div>
      {/if}
    </div>
    <button 
      type="button" 
      class="btn preset-filled-surface rounded-none"
      onclick={handleAbout}
    >
      About
    </button>
  </div>
</div>