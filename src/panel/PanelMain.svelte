<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
	import { PaneGroup, Pane, PaneResizer } from "paneforge";
  import { marked } from 'marked';
  import EasyMonacoEditor from '@cloudparker/easy-monaco-editor-svelte';
  import { rawText, compiledText } from '../lib/stores';
  

  let editorRef: HTMLDivElement;
  let containerRef: HTMLDivElement;
	let editor: any;
  
  const handleMonaco = (monaco: any) => {
    if (monaco && editorRef) {
      const getCSSColorValue = (variable: string) => {
        const el = document.createElement('div');
        el.style.color = `rgb(var(${variable}) / 1)`;
        document.body.appendChild(el);
        const color = getComputedStyle(el).color;
        document.body.removeChild(el);
        
        const rgb = color.match(/\d+/g);
        if (!rgb) return '#000000';
        const hex = '#' + rgb.map(x => {
          const hex = parseInt(x).toString(16);
          return hex.length === 1 ? '0' + hex : hex;
        }).join('');
        return hex;
      };

      editor = monaco.editor.create(editorRef, {
        value: $rawText,
        language: 'markdown',
        theme: 'vs-dark',
        fontSize: 14,
      });

      editor.getModel().onDidChangeContent(() => {
        $rawText = editor.getValue();
      });

      // Subscribe to rawText changes
      rawText.subscribe((newValue) => {
        if (editor && editor.getValue() !== newValue) {
          editor.setValue(newValue);
        }
      });

      const resizeObserver = new ResizeObserver(() => {
        editor.layout();
      });

      resizeObserver.observe(containerRef);

      return () => {
        resizeObserver.disconnect();
      };
    }
  }

  $effect(() => {
    return () => {
      editor && editor.dispose();
    }
  });
</script>

<div class="w-full h-full bg-surface-950">
  <PaneGroup direction="horizontal">
    <Pane>
      <div class="h-full w-full" bind:this={containerRef}>
        <EasyMonacoEditor onLoad={handleMonaco}>
          <div class="h-full w-full " bind:this={editorRef}></div>
        </EasyMonacoEditor>
      </div>
    </Pane>
    <PaneResizer class="w-1 bg-surface-800"/>
    <Pane>
      <div class="h-full w-full p-2 overflow-y-auto break-keep">
        <article class="prose prose-invert overflow-auto">
          {@html marked($compiledText)}
        </article>
      </div>
    </Pane>
  </PaneGroup>
</div>