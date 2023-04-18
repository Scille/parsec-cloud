import { Ref, ref } from 'vue';

const defaultWidth = 300;
const initialWidth = ref<number>(defaultWidth);
const computedWidth = ref<number>(defaultWidth);
let splitPane: Ref<any>;
let divider: Ref<any>;

export default function useSidebarMenu(): any {
  function init(iSplitPane: Ref<any>, iDivider: Ref<any>): void {
    splitPane = iSplitPane;
    divider = iDivider;
  }
  function isHidden(): boolean {
    return computedWidth.value === 4;
  }

  function reset(): void {
    resizeMenu(300);
    initialWidth.value = defaultWidth;
    computedWidth.value = defaultWidth;
  }

  function resizeMenu(newWidth: number): void {
    if (splitPane.value) {
      splitPane.value.$el.style.setProperty('--side-width', `${newWidth}px`);
    }
    if (divider.value) {
      divider.value.style.setProperty('left', `${newWidth - 4}px`);
    }
  }

  return {
    init,
    initialWidth,
    computedWidth,
    resizeMenu,
    isHidden,
    reset
  };
}
