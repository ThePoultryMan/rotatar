export type TwoInts = [number, number];

export type Config = {
  audio: AudioConfig;
  sections: TwoInts;
  idle_images: string[];
  speaking_images: string[];
  screen_information: ScreenInformation;
};
export type AudioConfig = {
  magnitude_threshold: number;
  max_magnitude: number;
};
export type ScreenInformation = {
  size: TwoInts;
  modifiers: Record<string, TwoInts>;
};

export type State = {
  current_image: number;
  sensitivity: number;
  // I don't know how this serializes because of the "Polling" state. So until that's figured out,
  // this will stay accepting any.
  audio_status: "Ready" | "Closed" | any;
  audio_devices: string[];
  section_size: TwoInts;
  x_sections: number;
};
