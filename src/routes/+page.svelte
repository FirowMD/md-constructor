<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
	import { PaneGroup, Pane, PaneResizer } from "paneforge";
  import PanelTopMenu from "../panel/PanelTopMenu.svelte";
  import PanelMain from "../panel/PanelMain.svelte";
  import PanelSide from "../panel/PanelSide.svelte";
</script>

<div class="flex flex-col w-full h-full bg-surface-900">
  <PanelTopMenu />
  <PaneGroup direction="vertical">
    <Pane>
      <PaneGroup direction="horizontal">
        <Pane defaultSize={20}>
          <PanelSide />
        </Pane> 
        <PaneResizer class="w-1 bg-surface-800"/>
        <Pane defaultSize={70}>
          <PanelMain />
        </Pane>
      </PaneGroup>
    </Pane>
  </PaneGroup>
</div>