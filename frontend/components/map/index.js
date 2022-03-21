import React, { useCallback } from 'react';
import { Point } from 'ol/geom';
import { fromLonLat, toLonLat, transformExtent } from 'ol/proj';
import 'ol/ol.css';
import { RMap, ROSM, RLayerVector, RFeature, RPopup, RStyle, RFill, RStroke } from 'rlayers';
import {} from 'rlayers/style';

export default function Home({ locations, setClick, setExtents }) {
  const center = fromLonLat([-100, 40.5]);

  return (
    <>
      <RMap
        height={'500px'}
        initial={{ center: center, zoom: 4 }}
        onClick={useCallback((e) => {
          const coords = e.map.getCoordinateFromPixel(e.pixel);
          const lonlat = toLonLat(coords);

          console.log(JSON.stringify(lonlat, null, 2));
          if (typeof setClick === 'function') {
            setClick(lonlat);
          }
        }, [])}
        onMoveEnd={useCallback((e) => {
          const extents = e.map.getView().calculateExtent(e.map.getSize());
          console.log(JSON.stringify(extents, null, 2));

          var extents_lonlat = transformExtent(extents, 'EPSG:3857', 'EPSG:4326'); // EPSG:4326 is like wgs84, lat/lon
          console.log(JSON.stringify(extents_lonlat, null, 2));
          if (typeof setExtents === 'function') {
            setExtents(extents_lonlat);
          }
        }, [])}
      >
        <ROSM />
        {locations && (
          <RLayerVector zIndex={10}>
            <RStyle.RStyle>
              <RStyle.RIcon src={'/fruit_icons/Apple.svg'} anchor={[0.5, 0.8]} />
            </RStyle.RStyle>
            {locations.map((location) => (
              <RFeature
                geometry={new Point(fromLonLat([location.longitude, location.latitude]))}
                onClick={(e) =>
                  e.map.getView().fit(e.target.getGeometry().getExtent(), {
                    duration: 250,
                    maxZoom: 8
                  })
                }
              >
                <RPopup trigger={'hover'} className="example-overlay">
                  <p>
                    <strong>{location.collection_title}</strong>
                  </p>
                </RPopup>
              </RFeature>
            ))}
          </RLayerVector>
        )}
      </RMap>
    </>
  );
}
