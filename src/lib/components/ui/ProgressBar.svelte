<script lang="ts">
  import { onDestroy } from "svelte";

  export let percentage: number = 0;

  onDestroy(() => (percentage = 0));
</script>

<div class={"progress-bar " + ($$restProps.class || "")}>
  <div style="width: {percentage}%; height: 100%;"></div>
</div>

<style lang="scss">
  .progress-bar {
    width: 24rem;
    height: 30px;
    background: $base;
    border-radius: 0px 0.4rem 0.4rem 0px;
    box-shadow: 0.4rem 0rem 0.5rem 0.2rem $shadow;
    overflow: hidden;
    position: relative;
    > div {
      background: $progressbar;
      transition: width 0.5s cubic-bezier(0.22, 0.61, 0.36, 1);
    }
    &.errored {
      background: $error;
    }
  }
</style>
