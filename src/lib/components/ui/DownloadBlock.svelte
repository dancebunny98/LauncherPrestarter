<script lang="ts">
  import ProgressBar from "./ProgressBar.svelte";

  export let error;
  export let speedMb;
  export let percentage: number;
  export let totalLabel = "";
</script>

<div data-tauri-drag-region class="download-block">
  <div class="speed-block">
    <div>
      {#if speedMb !== ""}
        <strong class={error ? "errored" : ""}>{speedMb}</strong>
        <small>Mbps</small>
      {:else}
        <strong>--</strong>
      {/if}
    </div>
    <small>{totalLabel}</small>
  </div>
  <ProgressBar class={error ? "errored" : ""} {percentage} />
</div>
{#if error !== null}
    <div class="error-label">
    {error}
  </div>
{/if}

<style lang="scss">
  @use "sass:math";
  .download-block {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
  }
  .speed-block {
    display: flex;
    width: 5rem;
    height: 80px;
    flex-direction: column;
    flex-wrap: nowrap;
    justify-content: center;
    align-items: center;
    gap: 0.375rem;
    border-radius: 0.75rem;
    box-shadow: 0px 5px 12px 6px $shadow;
    position: relative;

    > div {
      flex-direction: column;
      display: flex;
      align-items: center;

      > strong {
        font-size: 1.5rem;
        font-weight: 800;
        letter-spacing: 0.16px;
        color: transparent;
        background: $active;
        background-clip: text;
        text-shadow: none;
        text-align: center;
        &.errored {
          color: $error;
        }
      }

      > small {
        font-size: 0.75rem;
        color: $text-description;
        text-align: center;
      }
    }

    > small {
      font-size: 0.75rem;
      color: $text-secondary;
    }
  }
  .error-label {
    color: $error;
    text-align: center;
    font-size: 0.75rem;
  }
</style>
