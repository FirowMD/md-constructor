<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Accordion } from '@skeletonlabs/skeleton-svelte';
  import type { MarkdownElement } from '../lib/types';
  import { elements } from '../lib/stores';

  let content = $state("");
  const { element } = $props<{ element: MarkdownElement }>();
  
  $effect(() => {
    content = element.values[0] || "";
  });

  function handleInput() {
    const elementIndex = $elements.findIndex(e => e.element_name === element.element_name);
    if (elementIndex !== -1) {
      const updatedElement = { ...element };
      updatedElement.values = [content];
      updatedElement.labels = [content];
      updatedElement.chosens = [content];
      
      const newElements = [...$elements];
      newElements[elementIndex] = updatedElement;
      $elements = newElements;
    }
  }
</script>

<textarea 
  class="textarea p-2" 
  rows="4" 
  placeholder="Enter your text here..." 
  bind:value={content}
  oninput={handleInput}
></textarea>