<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Accordion } from '@skeletonlabs/skeleton-svelte';
  import type { MarkdownElement } from '../lib/types';
  import { elements } from '../lib/stores';

  const { element } = $props<{ element: MarkdownElement }>();
  let newValue = $state("");
  let optionsValue = $state<string[]>([]);

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
</script>

<select 
  class="select w-full"
  value={element.chosens[0] || ""}
  onchange={(e) => handleSelect(e.currentTarget.value)}
>
  <option value="" disabled>Select an option</option>
  {#each element.values as value, i}
    <option value={value}>{element.labels[i]}</option>
  {/each}
</select>

<Accordion value={optionsValue} onValueChange={(e) => (optionsValue = e.value)} multiple>
  <Accordion.Item controlClasses="text-sm" value={`${element.element_name}-options`}>
    {#snippet control()}
      Available Options
    {/snippet}
    {#snippet panel()}
      <div class="space-y-2">
        {#each element.values as value, i}
          <div class="flex items-center justify-between">
            <p>{element.labels[i]}</p>
            <button 
              type="button" 
              class="btn btn-sm variant-filled-error"
              onclick={() => removeOption(i)}
            >
              ✕
            </button>
          </div>
        {/each}
      </div>

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
