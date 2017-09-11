import pytest
import attr
from effect2 import Effect
from effect2.testing import perform_sequence, noop

from parsec.base import EventComponent, EEvent, ERegisterEvent, EUnregisterEvent


@attr.s
class RegisteredEventItent:
    event = attr.ib()
    sender = attr.ib()


class OtherRegisteredEventItent(RegisteredEventItent):
    pass


def _register_event(component, event='event1', sender='userA', intent_factory=RegisteredEventItent):
    eff = component.perform_register_event(ERegisterEvent(intent_factory, event, sender))
    sequence = [
    ]
    resp = perform_sequence(sequence, eff)
    assert resp is None
    return (event, sender, intent_factory)


@pytest.fixture
def component():
    component = EventComponent()
    _register_event(component, 'event1', None, RegisteredEventItent)
    _register_event(component, 'event1', 'userA', RegisteredEventItent)
    _register_event(component, 'event1', 'userA', OtherRegisteredEventItent)
    _register_event(component, 'event1', 'userB', RegisteredEventItent)
    _register_event(component, 'event2', 'userA', RegisteredEventItent)
    return component


def test_check_registered_events(component):
    assert component.listeners == {
        ('event1', None): [RegisteredEventItent],
        ('event1', 'userA'): [RegisteredEventItent, OtherRegisteredEventItent],
        ('event1', 'userB'): [RegisteredEventItent],
        ('event2', 'userA'): [RegisteredEventItent],
    }


def test_send_event_not_registered(component):
    eff = component.perform_trigger_event(EEvent('event42', 'userA'))
    sequence = [
    ]
    resp = perform_sequence(sequence, eff)
    assert resp is None


def test_register_event(component):
    eff = component.perform_register_event(ERegisterEvent(RegisteredEventItent, 'event42', 'userA'))
    sequence = [
    ]
    resp = perform_sequence(sequence, eff)
    assert resp is None
    assert RegisteredEventItent in component.listeners[('event42', 'userA')]


def test_unregister_event(component):
    assert component.listeners[('event1', 'userA')] == [
        RegisteredEventItent, OtherRegisteredEventItent]
    eff = component.perform_unregister_event(
        EUnregisterEvent(RegisteredEventItent, 'event1', 'userA'))
    sequence = [
    ]
    resp = perform_sequence(sequence, eff)
    assert resp is None
    assert component.listeners[('event1', 'userA')] == [OtherRegisteredEventItent]


def test_unregister_dont_shadow_other(component):
    # Unregister one shouldn't affect the others
    eff = component.perform_unregister_event(
        EUnregisterEvent(RegisteredEventItent, 'event1', 'userA'))
    sequence = [
    ]
    resp = perform_sequence(sequence, eff)
    assert resp is None
    assert component.listeners == {
        ('event1', None): [RegisteredEventItent],
        ('event1', 'userA'): [OtherRegisteredEventItent],
        ('event1', 'userB'): [RegisteredEventItent],
        ('event2', 'userA'): [RegisteredEventItent],
    }


def test_send_event_multiple_subscribers(component):
    eff = component.perform_trigger_event(EEvent('event1', 'userA'))
    sequence = [
        (RegisteredEventItent('event1', 'userA'), noop),
        (OtherRegisteredEventItent('event1', 'userA'), noop),
        (RegisteredEventItent('event1', 'userA'), noop),  # Event with None sender
    ]
    resp = perform_sequence(sequence, eff)
    assert resp is None
