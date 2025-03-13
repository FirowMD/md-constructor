<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Accordion } from '@skeletonlabs/skeleton-svelte';
  import type { MarkdownElement } from '../lib/types';
  import ElementTextbox from './ElementTextbox.svelte';
  import ElementCheckbox from './ElementCheckbox.svelte';
  import ElementRadio from './ElementRadio.svelte';
  import ElementList from './ElementList.svelte';
  import { elements } from '../lib/stores';
  import Type from 'lucide-svelte/icons/type';
  import ListChecks from 'lucide-svelte/icons/list-checks';
  import SquareAsterisk from 'lucide-svelte/icons/square-asterisk';
  import Rows3 from 'lucide-svelte/icons/rows-3';
  import ArrowUp from 'lucide-svelte/icons/arrow-up';
  import ArrowDown from 'lucide-svelte/icons/arrow-down';
  import Copy from 'lucide-svelte/icons/copy';

  const { element } = $props<{ element: MarkdownElement }>();
  let newName = $state(element.element_name);

  function getElementIcon(type: string) {
    switch (type) {
      case 'textbox':
        return Type;
      case 'checkbox':
        return ListChecks;
      case 'radio':
        return SquareAsterisk;
      case 'list':
        return Rows3;
      default:
        return Type;
    }
  }

  const Icon = $derived(getElementIcon(element.element_type));

  function getUniqueElementName(baseName: string): string {
    let counter = 1;
    let newName = baseName;
    
    while ($elements.some(e => e.element_name === newName)) {
      newName = `${baseName} ${counter}`;
      counter++;
    }
    
    return newName;
  }

  function handleRename() {
    const elementIndex = $elements.findIndex(e => e.element_name === element.element_name);
    if (elementIndex !== -1 && newName.trim()) {
      // Check if the new name would be unique (excluding the current element)
      const wouldBeDuplicate = $elements.some(e => 
        e.element_name === newName.trim() && e.element_name !== element.element_name
      );

      if (wouldBeDuplicate) {
        newName = getUniqueElementName(newName.trim());
      }

      const event = new CustomEvent('updateElementName', { 
        detail: { 
          oldName: element.element_name,
          newName: newName
        } 
      });
      window.dispatchEvent(event);

      const updatedElement = { ...element };
      updatedElement.element_name = newName;
      
      const newElements = [...$elements];
      newElements[elementIndex] = updatedElement;
      $elements = newElements;
    }
  }

  function handleDelete() {
    $elements = $elements.filter(e => e.element_name !== element.element_name);
  }

  function handleMoveUp() {
    const index = $elements.findIndex(e => e.element_name === element.element_name);
    if (index > 0) {
      const newElements = [...$elements];
      [newElements[index - 1], newElements[index]] = [newElements[index], newElements[index - 1]];
      $elements = newElements;
    }
  }

  function handleMoveDown() {
    const index = $elements.findIndex(e => e.element_name === element.element_name);
    if (index < $elements.length - 1) {
      const newElements = [...$elements];
      [newElements[index], newElements[index + 1]] = [newElements[index + 1], newElements[index]];
      $elements = newElements;
    }
  }

  function handleClone() {
    const baseName = element.element_name + ' (copy)';
    const clonedElement: MarkdownElement = {
      ...element,
      element_name: getUniqueElementName(baseName),
      is_collapsed: true
    };
    $elements = [...$elements, clonedElement];
  }
</script>

<Accordion.Item controlClasses="text-base" value={element.element_name}>
  {#snippet lead()}
    <Icon size={16} />
  {/snippet}
  {#snippet control()}
    {element.element_name}
  {/snippet}
  {#snippet panel()}
    <div class="flex flex-col gap-2 pt-2 pb-2">
      <input 
        class="input p-2 w-full" 
        type="text" 
        placeholder="Enter element name" 
        bind:value={newName}
      />
      <div class="flex flex-row gap-1 items-center">
        <button 
          type="button" 
          class="btn-icon variant-filled-surface"
          onclick={handleMoveUp}
          title="Move Up"
        >
          <ArrowUp size={16} />
        </button>
        <button 
          type="button" 
          class="btn-icon variant-filled-surface"
          onclick={handleMoveDown}
          title="Move Down"
        >
          <ArrowDown size={16} />
        </button>
        <button 
          type="button" 
          class="btn-icon variant-filled-surface"
          onclick={handleClone}
          title="Clone"
        >
          <Copy size={16} />
        </button>
        <div class="flex-grow"></div>
        <button 
          type="button" 
          class="btn preset-tonal-secondary"
          onclick={handleRename}
        >
          Rename
        </button>
        <button 
          type="button" 
          class="btn preset-tonal-tertiary"
          onclick={handleDelete}
        >
          Delete
        </button>
      </div>
    </div>
    {#if element.element_type === 'textbox'}
      <ElementTextbox element={element} />
    {:else if element.element_type === 'checkbox'}
      <ElementCheckbox element={element} />
    {:else if element.element_type === 'radio'}
      <ElementRadio element={element} />
    {:else if element.element_type === 'list'}
      <ElementList element={element} />
    {/if}
  {/snippet}
</Accordion.Item>
