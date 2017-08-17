import React from 'react';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import ZoneItem from './ZoneItem';
import { ActionCreators } from './actions';


ZonePicker.propTypes = {
    zones: PropTypes.object.isRequired
};

function mapStateToProps(state) {
    return {
        activeZone: state.activeZone,
        zones: state.zones
    };
}

function ZonePicker(props) {
    const listItems = props.zones
        .valueSeq()
        .sortBy(i => i.id)
        .map(item => <ZoneItem key={item.id}
                               activeZone={props.activeZone}
                               onActivate={props.updateActiveZone}
                               item={item}/>);

    const noneItem = { name: 'None', id: null, description: null };

    return (<ul className="zone-picker">
        <ZoneItem activeZone={props.activeZone}
                  onActivate={props.updateActiveZone}
                  item={noneItem}/>
        {listItems}
    </ul>);
}

export default connect(mapStateToProps, ActionCreators)(ZonePicker);