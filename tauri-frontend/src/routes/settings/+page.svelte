<script lang="ts">
  import { frontendData } from "$lib/stores.svelte";
  import { invoke } from "@tauri-apps/api/core";

  let audioDevice = $state();

  $effect(() => {
    if (
      audioDevice &&
      frontendData.config &&
      frontendData.state?.audio_devices[frontendData.config.audio.current_device] != audioDevice
    ) {
      invoke("set_audio_device", { device: audioDevice });
    }
  });
</script>

{#if frontendData.state}
  <select bind:value={audioDevice}>
    {#each frontendData.state.audio_devices as device}
      <option>{device}</option>
    {/each}
  </select>
{/if}
