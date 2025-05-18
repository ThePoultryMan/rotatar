<script lang="ts">
  const MAX_VALUE = 1.0;
  
  let { value }: { value: number } = $props();

  let percent = $derived(Math.round(value / MAX_VALUE * 100));
</script>

<div class="w-56" style={`background: linear-gradient(to right, #9c23d3 ${percent}%, #ffffff ${percent}%);`}></div>
