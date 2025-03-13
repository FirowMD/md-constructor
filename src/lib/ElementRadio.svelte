<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Accordion } from '@skeletonlabs/skeleton-svelte';
  import type { MarkdownElement } from '../lib/types';
  import { elements } from '../lib/stores';

  const { element } = $props<{ element: MarkdownElement }>();
  let newValue = $state("");
  let optionsValue = $state<string[]>([`${element.element_name}-options`]);

  function addOption() {
    if (newValue.trim()) {
      const elementIndex = $elements.findIndex(e => e.element_name === element.element_name);
      if (elementIndex !== -1) {
        const updatedElement = { ...element };
        updatedElement.values = [...updatedElement.values, newValue];
        updatedElement.labels = [...updatedElement.labels, newValue];
        
        const newElements = [...$elements];
        newElements[elementIndex] = updatedElement;
        $elements = newElements;
        newValue = "";
      }
    }
  }

  function handleSelect(value: string) {
    const elementIndex = $elements.findIndex(e => e.element_name === element.element_name);
    if (elementIndex !== -1) {
      const updatedElement = { ...element };
      updatedElement.chosens = [value];
      
      const newElements = [...$elements];
      newElements[elementIndex] = updatedElement;
      $elements = newElements;
    }
  }

  function removeOption(index: number) {
    const elementIndex = $elements.findIndex(e => e.element_name === element.element_name);
    if (elementIndex !== -1) {
      const updatedElement = { ...element };
      updatedElement.values = updatedElement.values.filter((_: string, i: number) => i !== index);
      updatedElement.labels = updatedElement.labels.filter((_: string, i: number) => i !== index);
      if (element.values[index] === element.chosens[0]) {
        updatedElement.chosens = [];
      }
      
      const newElements = [...$elements];
      newElements[elementIndex] = updatedElement;
      $elements = newElements;
    }
  }

  function handleValueChange(e: { value: string[] }) {
    optionsValue = e.value;
  }
</script>

<Accordion value={optionsValue} onValueChange={handleValueChange} multiple>
  <Accordion.Item controlClasses="text-sm" value={`${element.element_name}-options`}>
    {#snippet control()}
      Available Options
    {/snippet}
    {#snippet panel()}
      <form class="space-y-2">
        {#each element.values as value, i}
          <div class="flex items-center justify-between">
            <label class="flex items-center space-x-2">
              <input 
                class="radio" 
                type="radio" 
                name={element.element_name}
                checked={element.chosens[0] === value}
                oninput={() => handleSelect(value)}
              />
              <p>{element.labels[i]}</p>
            </label>
            <button 
              type="button" 
              class="btn btn-sm variant-filled-error"
              onclick={() => removeOption(i)}
            >
              ✕
            </button>
          </div>
        {/each}
      </form>

      <div class="flex flex-row h-10 p-1 gap-1 items-center mt-2">
        <input 
          class="input h-full p-2" 
          type="text" 
          placeholder="Enter option" 
          bind:value={newValue}
        />
        <button 
          type="button" 
          class="btn preset-tonal-primary h-full"
          onclick={addOption}
        >
          Add
        </button>
      </div>
    {/snippet}
  </Accordion.Item>
</Accordion>