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
