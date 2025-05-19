<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import "../app.css";
  import { onMount } from "svelte";
  import { frontendData } from "$lib/stores.svelte";
  import { convertFileSrc, invoke } from "@tauri-apps/api/core";
  import type { Config } from "$lib/types";

  let { children } = $props();

  listen<number>("current-image-changed", (event) => {
    if (frontendData.state) {
      console.log(event.payload);
      frontendData.state.current_image = event.payload;
    }
  });
  listen<number>("sensitivity-changed", (event) => {
    if (frontendData.state) {
      frontendData.state.sensitivity = event.payload;
    }
  });
  listen<Config>("config-changed", (event) => {
    frontendData.config = event.payload;
  });
  listen<string[]>("audio-devices-changed", (event) => {
    if (frontendData.state) {
      frontendData.state.audio_devices = event.payload;
    }
  });

  onMount(async () => {
    frontendData.config = await invoke("get_config");
    frontendData.state = await invoke("get_state");
  });
</script>

{@render children()}
