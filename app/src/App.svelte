<script lang="ts">
  import { query, configUrl } from "./store";
  import Timezone from "./lib/Timezone.svelte";
  import Reminder from "./lib/Reminder.svelte";

  const trimmedUtc = (date: Date) =>
    date.toISOString().replaceAll(/:\d\d\.\d\d\dZ/g, "");

  $: durationH = $query.duration && `${$query.duration}h`;

  function handleSubmit(e) {
    console.log(e);
    // alert(JSON.stringify(e));
    return e;
  }

  const default_title = undefined; // "Important Meeting";
  const default_location = undefined; // "Meeting Room Five";
  const default_start = trimmedUtc(new Date());
  const default_end = trimmedUtc(new Date());
  const default_duration = "01:00";
  const default_timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

  query.update((stored) => {
    console.log("updated", stored);

    const { start, end, duration, location, title, timezone } = stored;

    return {
      ...stored,
      start: start || default_start,
      end: end || default_end,
      title: title || default_title,
      timezone: timezone || default_timezone,
      location: location || default_location,
      duration: duration || default_duration,
    };
  });

  $: configUrlPretty = $configUrl
    .replace(/(&|\?)/g, "\n    $1")
    .replace(/(=)/g, " $1 ");
</script>

<main>
  
  <form on:submit={handleSubmit}>
    <table>
      <tr>
        <th> <label for="title"> Title </label> </th>
        <td
          ><input
            type="text"
            name="title"
            id="title"
            bind:value={$query.title}
            placeholder="title"
          /></td
        >
        <td class="preview">{$query.title}</td>
      </tr>

      <tr>
        <th>
          <label for="location"> location </label>
        </th>
        <td
          ><input
            type="text"
            name="location"
            id="location"
            placeholder="location"
            bind:value={$query.location}
          /></td
        >
        <td class="preview">{$query.location}</td>
      </tr>

      <!--

      <tr>
        <th>Categories</th>
        <td>(list with defaults)</td>
      </tr>

      <tr>
        <th>Attendees</th>
        <td>(list)</td>
      </tr>
      -->

      <tr>
        <th> <label for="start">start</label> </th>
        <td>
          <input
            type="datetime-local"
            id="start"
            name="start"
            bind:value={$query.start}
          />
        </td>
        <td class="preview">{$query.start}</td>
      </tr>

      <tr>
        <th> <label for="end">end</label> </th>
        <td>
          <input
            type="datetime-local"
            id="end"
            name="end"
            bind:value={$query.end}
          />
        </td>
        <td class="preview"> {$query.end} </td>
      </tr>

      <tr>
        <th><label for="duration">duration</label></th>
        <td>
          <input
            type="time"
            id="duration"
            name="duration"
            bind:value={$query.duration}
          /> (hh:mm)
        </td>
        <td class="preview"> {durationH} </td>
      </tr>

      <tr>
        <th><label for="timezone">timezone</label></th>
        <td> <Timezone bind:selectedZone={$query.timezone} /> </td>
        <td class="preview"> {$query.timezone} </td>
      </tr>

      <tr>
        <th><label for="reminder">reminder</label></th>
        <td> <Reminder bind:value={$query.reminder} /> </td>
        <td class="preview"> {JSON.stringify($query.reminder)} </td>
      </tr>
      <tr>
        <th />
        <td>
          <a href={"//" + location.host}>{location.host}</a>
          <a href={$configUrl}>permalink</a>
        </td>
        <td>
          <input type="submit" value="send" />
        </td>
      </tr>
    </table>
  </form>
</main>

<pre><code class="box">{JSON.stringify($query, null, 4)}</code></pre>
<pre><code class="box">{configUrlPretty}</code></pre>

<style>
  th {
    text-align: right;
  }
  td.preview {
    font-family: monospace;
  }
</style>
