<script lang="ts">
  import DynamicSlider from "$components/DynamicSlider.svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";

  let currentImage = $state("");
  let sensitivity = $state(0.0);

  listen<string>("current-image", (event) => {
    currentImage = convertFileSrc(event.payload);
  });
  listen<number>("sensitivity-changed", (event) => {
    sensitivity = event.payload;
  })
</script>

<div class="flex min-h-screen w-full items-center justify-center">
  <img src={currentImage} alt="current png" />
  <DynamicSlider value={sensitivity} />
  {sensitivity}
</div>
