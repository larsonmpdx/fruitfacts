import React from 'react';

export default function Home({ title, text, enabled, okFunction, cancelFunction }) {
  return enabled ? (
    <div class="bg-whitefixed fixed z-50 overflow-auto rounded-md p-5">
      <div class="rounded-lg bg-white p-8 shadow-2xl">
        <h2 class="text-lg font-bold">{title}</h2>

        <p class="mt-2 text-sm text-gray-500">{text}</p>

        <div class="mt-4 flex gap-2">
          <button
            type="button"
            onClick={okFunction}
            class="rounded bg-red-50 px-4 py-2 text-sm font-medium text-red-600"
          >
            Yes
          </button>

          <button
            type="button"
            onClick={cancelFunction}
            class="rounded bg-gray-50 px-4 py-2 text-sm font-medium text-gray-600"
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  ) : (
    <></>
  );
}
