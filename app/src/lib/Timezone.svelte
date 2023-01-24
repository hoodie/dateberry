<script lang="ts">
  // where I am
  const localTimezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

  // these timezones are known to the browser
  const browserTimezones = (Intl as any).supportedValuesOf("timeZone");

  // add these at least, in case browser doesn't have it (Chrome has no UTC 😲)
  const baseTimezones = ["UTC"];

  const allTimezones = Array.from(
    new Set([...baseTimezones, ...browserTimezones])
  );

  const allTimezonesGrouped = allTimezones
    .map((timezone: string) => [timezone, ...timezone.split(/\//)])
    .map(([timezone, group, ...suffix]) => [group, suffix.join("/"), timezone])
    .reduce(
      (map: Map<string, any>, [group, suffix, timezone]) =>
        map.has(group)
          ? map.set(group, [...map.get(group), { suffix, timezone }])
          : map.set(group, [{ suffix, timezone }]),
      new Map()
    );

  const timeZoneGroups = Array.from(allTimezonesGrouped.keys());

  let selectedGroup = localTimezone.split(/\//)[0];
  let timezonesInSelectedGroup = allTimezonesGrouped.get(selectedGroup);

  const groupHasZones = (timezonesInSelectedGroup) =>
    timezonesInSelectedGroup.filter(({ suffix }) => suffix.length).length > 0;

  const handleSelectGroup = ({ target: { value } }: any) => {
    timezonesInSelectedGroup = allTimezonesGrouped.get(value);
    if (!groupHasZones(timezonesInSelectedGroup)) {
      handleSelectZone({ target: { value } });
    }
  };

  const handleSelectZone = ({ target: { value } }: any) => {
    selectedZone = value;
  };

  export let selectedZone = localTimezone;
</script>

<select bind:value={selectedGroup} on:change={handleSelectGroup}>
  {#each timeZoneGroups as group}
    <option value={group}>
      {group}
    </option>
  {/each}
</select>

{#if groupHasZones(timezonesInSelectedGroup)}
  <select bind:value={selectedZone} on:change={handleSelectZone}>
    {#each timezonesInSelectedGroup as zone}
      <option value={zone.timezone}>
        {zone.suffix || zone.timezone}
      </option>
    {/each}
  </select>
{/if}
