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

<div class="mx-3 mt-3">
  <h1 class="mb-1 text-lg font-semibold">Settings</h1>
  <hr class="mb-3" />
  {#if frontendData.state}
    <label for="audio-device">Audio Device: </label>
    <select id="audio-device" bind:value={audioDevice}>
      {#each frontendData.state.audio_devices as device}
        <option>{device}</option>
      {/each}
    </select>
    <p class="text-sm">
      WARNING: Changing audio devices does not work well on all platform. If the app stops working
      after changing devices, restart it. This will be fixed in a later version.
    </p>
  {/if}
</div>
