export function locations_to_geoJSON (locations) {
  if (!Array.isArray(locations)) {
    return []
  }
  return locations.map(location => {
    return {
      type: 'Feature',
      properties: {
        cluster: false,
        collection_path: location.collection_path,
        collection_title: location.collection_title
      },
      geometry: {
        type: 'Point',
        coordinates: [location.longitude, location.latitude]
      }
    }
  })
}
