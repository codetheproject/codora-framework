import { useMDXComponents as getThemeComponents } from "nextra-theme-docs";

const themeComponents = getThemeComponents();

// @ts-expect-error Unknown type
export function useMDXComponents(components) {
	return {
		...themeComponents,
		...components,
	};
}
