<script lang="ts">
  import DynamicSlider from "$components/DynamicSlider.svelte";
  import type { Config } from "$lib/types";
  import { convertFileSrc, invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let config: Config | undefined = $state();
  let currentImage = $state("");
  let sensitivity = $state(0.0);
  let magnitude = $state(0);

  listen<string>("current-image", (event) => {
    currentImage = convertFileSrc(event.payload);
  });
  listen<number>("sensitivity-changed", (event) => {
    sensitivity = event.payload;
  });
  listen<number>("magnitude-changed", (event) => {
    magnitude = event.payload;
  });
  listen<any>("config-changed", (event) => {
    config = event.payload;
  });

  onMount(async () => {
    config = await invoke("get_config");
    currentImage = convertFileSrc(await invoke("get_current_image"));
  });

  $effect(() => {
    invoke("save_config", { config });
  });
</script>

{#if config}
  <div class="flex min-h-screen w-full items-center justify-center">
    <img src={currentImage} alt="current png" />
    <div class="*:my-2">
      <DynamicSlider value={sensitivity} />
      <DynamicSlider
        value={magnitude}
        max_value={config?.audio.max_magnitude}
        bind:threshold={config.audio.magnitude_threshold}
      />
    </div>
  </div>
{/if}
