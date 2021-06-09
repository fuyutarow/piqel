export const head7 = (shouldbeString: any) => (shouldbeString as string).slice(0, 7);

export const formatNumber = (num: number): string => {
  return num.toString().replace(/(\d)(?=(\d{3})+(?!\d))/g, '$1,');
};

export const getHostname = (href: string): string | null => {
  try {
    const url = new URL(href);
    return url.hostname;
  } catch (e) {
    return null;
  }
};

export const sleep = (ms: number) => {
  return new Promise(resolve => setTimeout(resolve, ms));
};

// (123.456, 0.1) => 123.5
export const round = (value: number, base: number) => {
  return Math.round(value / base) * base;
};
