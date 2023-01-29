<script lang="ts">
  import EventForm from "./lib/EventForm.svelte";
  import { query, configUrl } from "./store";

  $: configUrlPretty = $configUrl
    .replace(/(&|\?)/g, "\n    $1")
    .replace(/(=)/g, " $1 ");
</script>

<main>
  <header>
    <h1>DateBerry</h1>
  </header>

  <section class="eventform">
    <EventForm />
  </section>

  <aside class="debug">
    {#if $query.debug}
      <pre><code class="box">{JSON.stringify($query, null, 4)}</code></pre>
      <pre><code class="box">{configUrlPretty}</code></pre>
    {/if}
  </aside>

  <footer>
    <div class="box">
      <ul>
        <li>
          <a href={"//" + location.host}>reset ({location.host})</a>
        </li>
        <li>
          <a href={$configUrl}>store (permalink)</a>
        </li>
      </ul>
    </div>
  </footer>
</main>
