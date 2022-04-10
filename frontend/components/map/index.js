// see react-leaflet examples here https://tomik23.github.io/react-leaflet-examples/#/simple-map

import React, { useCallback } from 'react'
import Link from 'next/link'
import { MapContainer, Marker, Popup, TileLayer, useMapEvents, useMap } from 'react-leaflet'
import L from 'leaflet'
import useSupercluster from 'use-supercluster'
import 'leaflet/dist/leaflet.css'
import styles from '../../styles/Map.module.css'

import { locations_to_geoJSON } from './util'

function GetLocations ({ map, setClick, setExtents, setZoom }) {
  useMapEvents({
    click (e) {
      setClick(e.latlng)
    },
    locationfound (e) {
      console.log("got user's location") // todo
    }
  })

  const [bounds, setBounds] = React.useState([])

  React.useEffect(() => {
    if (!map) return

    setExtents(map.getBounds()) // initial
    setZoom(map.getZoom())

    map.on('moveend zoomend', () => {
      setExtents(map.getBounds())
      setZoom(map.getZoom())
    })
  }, [map])

  return <></>
}

const icons = {};
const getClusterIcon = (count, size) => {
  if (!icons[count]) {
    icons[count] = L.divIcon({
      html: `<div class="cluster-marker" style="width: ${size}px; height: ${size}px;">
        ${count}
      </div>`
    });
  }
  return icons[count];
};

const getFruitIcon = (type, size) => {
  if (!icons[type]) {
    icons[type] = L.divIcon({
      html: `<svg src={'/fruit_icons/${type}.svg'} style="width: ${size}px; height: ${size}px;">
        ${type}
      </svg>`
    });
  }
  return icons[type];
};

export default function Home ({ locations, setClick, setExtentsForFetch }) {
  const [extents, setExtents] = React.useState(null)
  const [clusterBounds, setClusterBounds] = React.useState(null)
  const [zoom, setZoom] = React.useState(3)

  React.useEffect(() => {
    if (!extents) {
      return
    }

    let bounds = [
      extents._southWest.lng,
      extents._southWest.lat,
      extents._northEast.lng,
      extents._northEast.lat
    ]
    setExtentsForFetch(bounds)

    // convert leaflet extents to the bounds format supercluster wants
    setClusterBounds(bounds)
  }, [extents])

  let locations_geoJSON = locations_to_geoJSON(locations)
  const { clusters, supercluster } = useSupercluster({
    points: locations_geoJSON,
    bounds: clusterBounds,
    zoom,
    options: { radius: 75, maxZoom: 20 }
  })

  console.log('clusters: ' + JSON.stringify(clusters, null, 2))

  const [map, setMap] = React.useState(null)

  return (
    <MapContainer
      zoom={3}
      scrollWheelZoom={true}
      style={{ height: 400, width: '100%' }}
      center={[40.5, -100]}
      whenCreated={setMap}
    >
      <TileLayer
        attribution='&copy; <a href="http://osm.org/copyright">OpenStreetMap</a> contributors'
        url='https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png'
      />
      <GetLocations map={map} setClick={setClick} setExtents={setExtents} setZoom={setZoom} />

      {clusters.map(cluster => {
        const [longitude, latitude] = cluster.geometry.coordinates
        const { cluster: isCluster, point_count: pointCount } = cluster.properties

        if (isCluster) {
          return (
            <Marker
              key={`cluster-${cluster.id}`}
              position={[latitude, longitude]}
               icon={getClusterIcon(
                pointCount,
                 10 + (pointCount / locations_geoJSON.length) * 40
               )}
            />
          )
        }

        return (
          <Marker
            key={`point-${cluster.properties.collection_path}`}
            position={[latitude, longitude]}
            icon={getFruitIcon(
              "Apple",
               20
             )}
          />
        )
      })}
    </MapContainer>
  )
}
