<script lang="ts">
  import EventForm from "./lib/EventForm.svelte";
  import { query, configUrl, configQuery } from "./store";

  $: hrefIcs = "//" + location.host + "/calendar.ics" + $configQuery;
  $: hrefTxt = "//" + location.host + "/calendar.txt" + $configQuery;
  $: configUrlPretty = $configUrl
    .replace(/(&|\?)/g, "\n    $1")
    .replace(/(=)/g, " $1 ");
</script>

<main>
  <header>
    <h1>
      <img src="/calendar.svg" alt="today's calendar" />
      DateBerry
    </h1>
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
        {#if $query.summary}
          <li>
            <a href={hrefTxt}>calendar.txt</a>
          </li>
          <li>
            <a href={hrefIcs}>calendar.ics</a>
          </li>
        {/if}
      </ul>
    </div>
  </footer>
</main>

<style>
  h1 > img {
    height: 1em;
    vertical-align: top;
  }
</style>
