<script lang="ts">
  type Props = {
    max_value?: number;
    value: number;
    threshold?: number;
  };

  let { max_value = 1.0, value, threshold = $bindable() }: Props = $props();

  let percent = $derived(Math.round((value / max_value) * 100));
</script>

<input
  dir="rtl"
  type="range"
  class="block h-3 w-56 appearance-none border-1"
  bind:value={threshold}
  step="0.01"
  max={max_value}
  style={`--percent: ${percent}%`}
/>

<style lang="postcss">
  @reference "../../app.css";

  input[type="range"]::-webkit-slider-runnable-track {
    @apply h-2.5;
    background: linear-gradient(to left, #9c23d3 var(--percent), #ffffff var(--percent));
  }

  input[type="range"]::-webkit-slider-thumb {
    @apply h-5 w-1 appearance-none bg-sky-500;
    margin-top: calc((var(--spacing) * 2.5 / 2) - (var(--spacing) * 5 / 2));
  }
</style>
