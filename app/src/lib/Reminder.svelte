<script lang="ts">
  interface Reminder {
    title: string;
    duration: { hours?: number; minutes?: number } | null;
  }
  const reminders: Array<Reminder> = [
    { title: "none", duration: null },
    { title: "5min before", duration: { minutes: 5 } },
    { title: "10min before", duration: { minutes: 10 } },
    { title: "15min before", duration: { minutes: 15 } },
    { title: "half an hour before", duration: { minutes: 30 } },
    { title: "1 hour before", duration: { hours: 1 } },
    { title: "2 hours before", duration: { hours: 2 } },
    { title: "6 hours before", duration: { hours: 6 } },
  ];
  export let value;
</script>

<select name="reminder" id="reminder" bind:value>
  {#each reminders as reminder}
    <option value={reminder.duration}>{reminder.title}</option>
  {/each}
</select>
