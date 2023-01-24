import { derived, writable } from 'svelte/store';

const readObjectFromQuery = (): Record<string, any> => {
    const queryEntries = Object.fromEntries(new URLSearchParams(location.search).entries());
    return ({
        ...queryEntries
    })
};

export const query = writable(readObjectFromQuery());

export const queryStringify = (entries: ReturnType<typeof Object.entries>) => {
    const params = new URLSearchParams();
    for (let [key, val] of entries) {
        if (val) params.set(key, JSON.stringify(val));
    }
    return params.toString();
};

const objectToUrl = (config: object) => `${window.origin}${window.location.pathname}?${queryStringify(Object.entries(config))}`

export const configUrl = derived(
    query,
    objectToUrl
);