import React, { useCallback } from 'react'
import Link from 'next/link'
import { MapContainer, Marker, Popup, TileLayer, useMapEvents, useMap } from 'react-leaflet'
import L from 'leaflet'
import 'leaflet/dist/leaflet.css'
import styles from '../../styles/Map.module.css'

import { locations_to_geoJSON } from './util'

function GetLocations ({ map, setClick, setExtents }) {
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

    map.on('moveend zoomend', () => {
      setExtents(map.getBounds())
    })
  }, [map])

  return <></>
}

export default function Home ({ locations, setClick, setExtents }) {
  let locations_geoJSON = locations_to_geoJSON(locations)

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
      <GetLocations map={map} setClick={setClick} setExtents={setExtents} />
    </MapContainer>
  )
}
