use conductor::manager::{AudioManager, AudioManagerSettings, Event, LoopSettings};
use mlua::prelude::*;

use crate::{
	duration::LDuration, error::ConductorLuaError, event::CustomEvent, event::LEvent,
	instance::LInstanceId, instance::LInstanceSettings, metronome::LMetronomeSettings,
	sequence::LSequence, sequence::LSequenceId, sound::LSoundId, sound::LSoundSettings,
	tempo::LTempo, tween::LTween,
};

pub struct LLoopSettings(pub LoopSettings);

impl<'lua> FromLua<'lua> for LLoopSettings {
	fn from_lua(lua_value: LuaValue<'lua>, _: &'lua Lua) -> LuaResult<Self> {
		match lua_value {
			LuaNil => Ok(LLoopSettings(LoopSettings::default())),
			LuaValue::Table(table) => {
				let mut settings = LoopSettings::default();
				if table.contains_key("startPoint")? {
					settings.start = Some(table.get::<_, LDuration>("startPoint")?.0);
				}
				if table.contains_key("endPoint")? {
					settings.end = Some(table.get::<_, LDuration>("endPoint")?.0);
				}
				Ok(LLoopSettings(settings))
			}
			value => Err(LuaError::external(ConductorLuaError::wrong_argument_type(
				"sequence", "table", value,
			))),
		}
	}
}

pub struct LAudioManagerSettings(pub AudioManagerSettings);

impl<'lua> FromLua<'lua> for LAudioManagerSettings {
	fn from_lua(lua_value: LuaValue<'lua>, _: &'lua Lua) -> LuaResult<Self> {
		match lua_value {
			LuaNil => Ok(LAudioManagerSettings(AudioManagerSettings::default())),
			LuaValue::Table(table) => {
				let mut settings = AudioManagerSettings::default();
				if table.contains_key("numCommands")? {
					settings.num_commands = table.get("numCommands")?;
				}
				if table.contains_key("numEvents")? {
					settings.num_events = table.get("numEvents")?;
				}
				if table.contains_key("numSounds")? {
					settings.num_sounds = table.get("numSounds")?;
				}
				if table.contains_key("numInstances")? {
					settings.num_instances = table.get("numInstances")?;
				}
				if table.contains_key("numSequences")? {
					settings.num_sequences = table.get("numSequences")?;
				}
				if table.contains_key("metronomeSettings")? {
					settings.metronome_settings =
						table.get::<_, LMetronomeSettings>("metronomeSettings")?.0;
				}
				Ok(LAudioManagerSettings(settings))
			}
			value => Err(LuaError::external(ConductorLuaError::wrong_argument_type(
				"audio manager settings",
				"table",
				value,
			))),
		}
	}
}

pub struct LAudioManager(pub AudioManager<CustomEvent>);

impl LAudioManager {
	pub fn new(settings: LAudioManagerSettings) -> LuaResult<Self> {
		match AudioManager::new(settings.0) {
			Ok(manager) => Ok(Self(manager)),
			Err(error) => Err(LuaError::external(error)),
		}
	}
}

impl LuaUserData for LAudioManager {
	fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
		methods.add_method_mut(
			"loadSound",
			|_: &Lua, this: &mut Self, (path, settings): (LuaString, LSoundSettings)| match this
				.0
				.load_sound(path.to_str()?, settings.0)
			{
				Ok(id) => Ok(LSoundId(id)),
				Err(error) => Err(LuaError::external(error)),
			},
		);

		methods.add_method_mut(
			"unloadSound",
			|_: &Lua, this: &mut Self, id: LSoundId| match this.0.unload_sound(id.0) {
				Ok(_) => Ok(()),
				Err(error) => Err(LuaError::external(error)),
			},
		);

		methods.add_method_mut(
			"playSound",
			|_: &Lua, this: &mut Self, (sound_id, settings): (LSoundId, LInstanceSettings)| {
				match this.0.play_sound(sound_id.0, settings.0) {
					Ok(id) => Ok(LInstanceId(id)),
					Err(error) => Err(LuaError::external(error)),
				}
			},
		);

		methods.add_method_mut(
			"setInstanceVolume",
			|_: &Lua, this: &mut Self, (id, volume, tween): (LInstanceId, f64, Option<LTween>)| {
				match this
					.0
					.set_instance_volume(id.0, volume, tween.map(|tween| tween.0))
				{
					Ok(_) => Ok(()),
					Err(error) => Err(LuaError::external(error)),
				}
			},
		);

		methods.add_method_mut(
			"setInstancePitch",
			|_: &Lua, this: &mut Self, (id, pitch, tween): (LInstanceId, f64, Option<LTween>)| {
				match this
					.0
					.set_instance_pitch(id.0, pitch, tween.map(|tween| tween.0))
				{
					Ok(_) => Ok(()),
					Err(error) => Err(LuaError::external(error)),
				}
			},
		);

		methods.add_method_mut(
			"pauseInstance",
			|_: &Lua, this: &mut Self, (id, fade_tween): (LInstanceId, Option<LTween>)| match this
				.0
				.pause_instance(id.0, fade_tween.map(|tween| tween.0))
			{
				Ok(_) => Ok(()),
				Err(error) => Err(LuaError::external(error)),
			},
		);

		methods.add_method_mut(
			"resumeInstance",
			|_: &Lua, this: &mut Self, (id, fade_tween): (LInstanceId, Option<LTween>)| match this
				.0
				.resume_instance(id.0, fade_tween.map(|tween| tween.0))
			{
				Ok(_) => Ok(()),
				Err(error) => Err(LuaError::external(error)),
			},
		);

		methods.add_method_mut(
			"stopInstance",
			|_: &Lua, this: &mut Self, (id, fade_tween): (LInstanceId, Option<LTween>)| match this
				.0
				.stop_instance(id.0, fade_tween.map(|tween| tween.0))
			{
				Ok(_) => Ok(()),
				Err(error) => Err(LuaError::external(error)),
			},
		);

		methods.add_method_mut(
			"pauseInstancesOfSound",
			|_: &Lua, this: &mut Self, (id, fade_tween): (LSoundId, Option<LTween>)| match this
				.0
				.pause_instances_of_sound(id.0, fade_tween.map(|tween| tween.0))
			{
				Ok(_) => Ok(()),
				Err(error) => Err(LuaError::external(error)),
			},
		);

		methods.add_method_mut(
			"resumeInstancesOfSound",
			|_: &Lua, this: &mut Self, (id, fade_tween): (LSoundId, Option<LTween>)| match this
				.0
				.resume_instances_of_sound(id.0, fade_tween.map(|tween| tween.0))
			{
				Ok(_) => Ok(()),
				Err(error) => Err(LuaError::external(error)),
			},
		);

		methods.add_method_mut(
			"stopInstancesOfSound",
			|_: &Lua, this: &mut Self, (id, fade_tween): (LSoundId, Option<LTween>)| match this
				.0
				.stop_instances_of_sound(id.0, fade_tween.map(|tween| tween.0))
			{
				Ok(_) => Ok(()),
				Err(error) => Err(LuaError::external(error)),
			},
		);

		methods.add_method_mut(
			"setMetronomeTempo",
			|_: &Lua, this: &mut Self, tempo: LTempo| match this.0.set_metronome_tempo(tempo.0) {
				Ok(_) => Ok(()),
				Err(error) => Err(LuaError::external(error)),
			},
		);

		methods.add_method_mut(
			"startMetronome",
			|_: &Lua, this: &mut Self, _: ()| match this.0.start_metronome() {
				Ok(_) => Ok(()),
				Err(error) => Err(LuaError::external(error)),
			},
		);

		methods.add_method_mut(
			"pauseMetronome",
			|_: &Lua, this: &mut Self, _: ()| match this.0.pause_metronome() {
				Ok(_) => Ok(()),
				Err(error) => Err(LuaError::external(error)),
			},
		);

		methods.add_method_mut(
			"stopMetronome",
			|_: &Lua, this: &mut Self, _: ()| match this.0.stop_metronome() {
				Ok(_) => Ok(()),
				Err(error) => Err(LuaError::external(error)),
			},
		);

		methods.add_method_mut(
			"startSequence",
			|_: &Lua, this: &mut Self, sequence: LSequence| match this.0.start_sequence(sequence.0)
			{
				Ok(id) => Ok(LSequenceId(id)),
				Err(error) => Err(LuaError::external(error)),
			},
		);

		methods.add_method_mut(
			"loopSound",
			|_: &Lua,
			 this: &mut Self,
			 (sound_id, loop_settings, instance_settings): (
				LSoundId,
				LLoopSettings,
				LInstanceSettings,
			)| {
				match this
					.0
					.loop_sound(sound_id.0, loop_settings.0, instance_settings.0)
				{
					Ok(id) => Ok(LSequenceId(id)),
					Err(error) => Err(LuaError::external(error)),
				}
			},
		);

		methods.add_method_mut(
			"muteSequence",
			|_: &Lua, this: &mut Self, id: LSequenceId| match this.0.mute_sequence(id.0) {
				Ok(_) => Ok(()),
				Err(error) => Err(LuaError::external(error)),
			},
		);

		methods.add_method_mut(
			"unmuteSequence",
			|_: &Lua, this: &mut Self, id: LSequenceId| match this.0.unmute_sequence(id.0) {
				Ok(_) => Ok(()),
				Err(error) => Err(LuaError::external(error)),
			},
		);

		methods.add_method_mut(
			"getEvents",
			|lua: &Lua, this: &mut Self, callbacks: Option<LuaTable>| {
				if let Some(callbacks) = callbacks {
					for event in this.0.events() {
						match event {
							Event::MetronomeIntervalPassed(interval) => {
								if callbacks.contains_key("metronomeIntervalPassed")? {
									callbacks.call_function("metronomeIntervalPassed", interval)?;
								}
							}
							Event::Custom(event) => {
								if callbacks.contains_key("custom")? {
									callbacks.call_function("custom", event)?;
								}
							}
						}
					}
					Ok(LuaValue::Nil)
				} else {
					Ok(LuaValue::Table(lua.create_sequence_from(
						this.0.events().iter().map(|event| LEvent(*event)),
					)?))
				}
			},
		);

		methods.add_method_mut("freeUnusedResources", |_: &Lua, this: &mut Self, _: ()| {
			Ok(this.0.free_unused_resources())
		})
	}
}
