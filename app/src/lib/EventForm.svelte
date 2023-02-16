<script lang="ts">
  import { query, configUrl, configQuery } from "../store";
  import Timezone from "./Timezone.svelte";
  import Reminder from "./Reminder.svelte";

  const trimmedUtc = (date: Date) =>
    date.toISOString().replaceAll(/:\d\d\.\d\d\dZ/g, "");

  // $: durationH = $query.duration && `${$query.duration}h`;

  function handleSubmit(e) {
    console.log(e);
    // alert(JSON.stringify(e));
    return e;
  }

  const default_summary = undefined; // "Important Meeting";
  const default_location = undefined; // "Meeting Room Five";
  const default_description = undefined; // "Meeting Room Five";
  const default_start = trimmedUtc(new Date());
  const default_end = trimmedUtc(new Date());
  // const default_duration = "01:00";
  const default_timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

  query.update((stored) => {
    console.log("updated", stored);

    const { start, end, location, description, summary, timezone } = stored;

    return {
      ...stored,
      start: start || default_start,
      end: end || default_end,
      summary: summary || default_summary,
      description: description || default_description,
      timezone: timezone || default_timezone,
      location: location || default_location,
    };
  });

  $: configUrlPretty = $configUrl
    .replace(/(&|\?)/g, "\n    $1")
    .replace(/(=)/g, " $1 ");
</script>

<form on:submit={handleSubmit}>
  <table>
    <tr>
      <th> <label for="summary"> summary </label> </th>
      <td
        ><input
          type="text"
          name="summary"
          id="summary"
          bind:value={$query.summary}
          placeholder="summary"
        /></td
      >
      <td class="preview" class:hidden={!$query.debug}>{$query.summary}</td>
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
      <td class="preview" class:hidden={!$query.debug}>{$query.location}</td>
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
      <td class="preview" class:hidden={!$query.debug}>{$query.start}</td>
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
      <td class="preview" class:hidden={!$query.debug}> {$query.end} </td>
    </tr>

    <!--
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
      <td class="preview" class:hidden={!$query.debug}> {durationH} </td>
    </tr>
    -->

    <tr>
      <th><label for="timezone">timezone</label></th>
      <td>
        <Timezone bind:selectedZone={$query.timezone} />
        <input
          type="hidden"
          name="timezone"
          id="timezone"
          bind:value={$query.timezone}
          readonly
        />
      </td>
      <td class="preview" class:hidden={!$query.debug}> {$query.timezone} </td>
    </tr>

    <tr>
      <th><label for="reminder">reminder</label></th>
      <td colspan="2"
        ><textarea
          name="description"
          id=""
          cols="30"
          rows="10"
          bind:value={$query.description}
        /></td
      >
    </tr>

    <tr>
      <th><label for="reminder">reminder</label></th>
      <td> <Reminder bind:value={$query.reminder} /> </td>
      <td class="preview" class:hidden={!$query.debug}>
        {JSON.stringify($query.reminder)}
      </td>
    </tr>

    <tr>
      <th><label for="debug">debug</label></th>
      <td
        ><input
          type="checkbox"
          name="debug"
          id="debug"
          bind:checked={$query.debug}
        /></td
      >
      <td class="preview" class:hidden={!$query.debug}>
        {JSON.stringify($query.debug)}
      </td>
    </tr>

    <tr>
      <th />
      <td>
        <input type="submit" value="store" />
      </td>
    </tr>
  </table>
</form>

<style>
  th {
    text-align: right;
  }
  .hidden {
    background-color: red;
    display: none;
  }
  td.preview {
    font-family: monospace;
  }
</style>
