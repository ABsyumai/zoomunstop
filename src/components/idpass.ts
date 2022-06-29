export interface IdPassProps {
  confno: string;
  pwd: string;
}

export const emits = [
  "update:confno",
  "update:pwd"
]