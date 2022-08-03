export const convert_seconds_to_hhmmss = (seconds: number) =>
  new Date(seconds * 1000).toISOString().slice(14, 19);

export const convert_date_to_full_readable_date = (created: Date) => {
  const date = new Date(created);

  const day = date.getDate();
  const month = date.getMonth();
  const year = date.getFullYear();

  return `${day}.${month}.${year}`;
}