export function locations_to_geoJSON(locations) {
  if (!Array.isArray(locations)) {
    return [];
  }
  return locations.map((location) => {
    return {
      type: 'Feature',
      properties: {
        cluster: false,
        ...location
      },
      geometry: {
        type: 'Point',
        coordinates: [location.longitude, location.latitude]
      }
    };
  });
}
