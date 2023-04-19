import { ref } from 'vue';

const defaultWidth = 300;
const initialWidth = ref<number>(defaultWidth);
const computedWidth = ref<number>(defaultWidth);
const wasReset = ref<boolean>(false);

export default function useSidebarMenu(): any {
  function isHidden(): boolean {
    return computedWidth.value === 4;
  }

  function reset(): void {
    initialWidth.value = defaultWidth;
    computedWidth.value = defaultWidth;
    wasReset.value = true;
  }

  return {
    defaultWidth,
    initialWidth,
    computedWidth,
    isHidden,
    reset,
    wasReset
  };
}
