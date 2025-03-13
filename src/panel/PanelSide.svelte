<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Accordion } from '@skeletonlabs/skeleton-svelte';
  import type { MarkdownElement } from '../lib/types';
  import SideElement from '../lib/SideElement.svelte';
  import { elements } from '../lib/stores';

  let selectedElementType = $state("textbox");
  let value = $state<string[]>([]);
  let id = $state(0);

  const imgSrc = 'assets/logo.png';

  // Listen for updates from file open
  $effect(() => {
    const handleUpdateAccordion = (event: CustomEvent<{ value: string[] }>) => {
      value = event.detail.value;
    };

    window.addEventListener('updateAccordion', handleUpdateAccordion as EventListener);
    return () => {
      window.removeEventListener('updateAccordion', handleUpdateAccordion as EventListener);
    };
  });

  // Update is_collapsed when accordion state changes
  $effect(() => {
    const newElements = $elements.map(element => ({
      ...element,
      is_collapsed: !value.includes(element.element_name)
    }));
    if (JSON.stringify(newElements) !== JSON.stringify($elements)) {
      $elements = newElements;
    }
  });

  // Update value array when loading elements with is_collapsed state
  $effect(() => {
    const expandedElements = $elements
      .filter(element => !element.is_collapsed)
      .map(element => element.element_name);
    if (JSON.stringify(expandedElements) !== JSON.stringify(value)) {
      value = expandedElements;
    }
  });

  // Listen for element name updates
  $effect(() => {
    const handleUpdateElementName = (event: CustomEvent<{ oldName: string, newName: string }>) => {
      const { oldName, newName } = event.detail;
      if (value.includes(oldName)) {
        value = value.map(v => v === oldName ? newName : v);
      }
    };

    window.addEventListener('updateElementName', handleUpdateElementName as EventListener);
    return () => {
      window.removeEventListener('updateElementName', handleUpdateElementName as EventListener);
    };
  });

  function getUniqueElementName(baseName: string): string {
    let counter = 1;
    let newName = baseName;
    
    while ($elements.some(e => e.element_name === newName)) {
      newName = `${baseName} ${counter}`;
      counter++;
    }
    
    return newName;
  }

  async function addElement() {
    const element: MarkdownElement = {
      element_name: getUniqueElementName("New element"),
      element_type: selectedElementType,
      labels: [],
      values: [],
      chosens: [],
      is_collapsed: true,
    };
    id++;
    $elements = [...$elements, element];
  }
</script>

<div class="flex flex-col w-full h-full bg-surface-900 overflow-y-auto">
  <div class="flex flex-row gap-8 items-end bg-surface-900 border-surface-200-800 w-full border-[1px] p-4 rounded-none">
    <img src={imgSrc} class="w-10 h-10" alt="logo" />
    <h1 class="h6 truncate">MD Constructor v.1</h1>
  </div>
  <Accordion value={value} onValueChange={(e) => (value = e.value)} multiple>
    {#each $elements as element (element.element_name)}
      <SideElement element={element} />
    {/each}
  </Accordion>

  <div class="flex flex-row h-10 p-1 gap-1 items-center">
    <label class="label h-full">
      <select class="select h-full" bind:value={selectedElementType}>
        <option value="textbox">Textbox</option>
        <option value="checkbox">Checkbox</option>
        <option value="radio">Radio</option>
        <option value="list">List</option>
      </select>
    </label>
    <button onclick={addElement} type="button" class="btn preset-tonal-primary h-full">Add</button>
  </div>
</div>