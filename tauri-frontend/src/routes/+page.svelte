<script lang="ts">
  import { onDestroy } from "svelte";

  import DynamicSlider from "$components/DynamicSlider.svelte";

  import IconSettingsOutlineRounded from "~icons/material-symbols/settings-outline-rounded";

  import { convertFileSrc, invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { frontendData } from "$lib/stores.svelte";

  let currentImage = $derived.by(() => {
    if (frontendData.config && frontendData.state) {
      let image;
      if (frontendData.state.sensitivity > 0) {
        image = frontendData.config.speaking_images[frontendData.state.current_image];
      } else {
        image = frontendData.config.idle_images[frontendData.state.current_image];
      }
      return convertFileSrc(image);
    } else {
      ("");
    }
  });
  let magnitude = $state(0);

  let listeners = [];
  listeners[0] = listen<number>("magnitude-changed", (event) => {
    magnitude = event.payload;
  });

  onDestroy(async () => {
    listeners.forEach(async (unlisten) => {
      (await unlisten)();
    });
  });

  $effect(() => {
    // invoke("save_config", { config: frontendData.config });
  });
</script>

{#if frontendData.config && frontendData.state}
  <a href="/settings" class="absolute right-2 top-2 block">
    <IconSettingsOutlineRounded style="font-size: calc(var(--spacing) * 6)" />
  </a>
  <div class="flex min-h-screen w-full items-center justify-center">
    <img src={currentImage} alt="current png" />
    <div class="*:my-2">
      <DynamicSlider value={frontendData.state.sensitivity} threshold={0} />
      <DynamicSlider
        value={magnitude}
        max_value={frontendData.config.audio.max_magnitude}
        bind:threshold={frontendData.config.audio.magnitude_threshold}
      />
    </div>
  </div>
{/if}
