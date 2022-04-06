// This file is a updated version of https://github.com/evalphobia/logrus_sentry:
//
// - Uses sentry_go rather than raven_go (which is deprecated and no longer maintained)
// - (only for charted-server) Doesn't append a new Sentry client, it uses the one in the global container if not nil.
//
// Code is released under MIT License (https://github.com/evalphobia/logrus_sentry/blob/master/LICENSE); all credits
// go to the author and not Noelware.

package loghooks

import (
	"errors"
	"sync"

	"github.com/getsentry/sentry-go"
	"github.com/sirupsen/logrus"
)

// SentryHook delivers logs to a Sentry instance, on the official SaaS platform or
// self-hosted.
type SentryHook struct {
	mu       *sync.RWMutex
	client   *sentry.Client
	modifier sentry.EventModifier
}

type defaultEventModifier struct{}

func newDefaultModifier() sentry.EventModifier {
	return &defaultEventModifier{}
}

func (*defaultEventModifier) ApplyToEvent(event *sentry.Event, _ *sentry.EventHint) *sentry.Event {
	return event
}

func NewSentryHook(client *sentry.Client) logrus.Hook {
	return &SentryHook{
		mu:       &sync.RWMutex{},
		client:   client,
		modifier: newDefaultModifier(),
	}
}

func isErrorLevel(level logrus.Level) bool {
	for _, value := range []logrus.Level{logrus.ErrorLevel, logrus.FatalLevel, logrus.PanicLevel} {
		if level == value {
			return true
		}
	}

	return false
}

func (hook *SentryHook) Levels() []logrus.Level {
	return logrus.AllLevels
}

func (hook *SentryHook) Fire(entry *logrus.Entry) error {
	// Allows multiple goroutines to log simultaneously.
	hook.mu.RLock()
	defer hook.mu.RUnlock()

	// Create the fields if applied
	fields := make(logrus.Fields)
	for k, v := range entry.Data {
		fields[k] = v
	}

	// Check if we are not under errors
	if !isErrorLevel(entry.Level) {
		// Append it as a breadcrumb and just skip it
		// Get level
		level := sentry.LevelInfo
		switch entry.Level {
		case logrus.InfoLevel:
			break

		case logrus.WarnLevel:
			level = sentry.LevelWarning
			break

		case logrus.DebugLevel:
			level = sentry.LevelDebug
			break

		default:
			panic("We shouldn't be able to reach here. :(")
		}

		// event := sentry.NewEvent()
		brebcrumb := &sentry.Breadcrumb{
			Message: entry.Message,
			Level:   level,
		}

		if ca, ok := fields["category"].(string); ok {
			brebcrumb.Category = ca
		}

		// event.Breadcrumbs = append(event.Breadcrumbs, brebcrumb)
		// event.Timestamp = time.Now()
		// event.Contexts = map[string]any{
		// 	"product": map[string]any{
		// 		"vendor": "Noelware",
		// 		"name":   "charted-server",
		// 	},
		// }

		// event.Platform = "go"
		// event.ServerName = hook.client.Options().ServerName
		// event.Release = hook.client.Options().Release
		// event.Message = entry.Message

		// hook.client.CaptureEvent(event, nil, hook.modifier)
		sentry.AddBreadcrumb(brebcrumb)
		return nil
	}

	level := sentry.LevelError
	if entry.Level == logrus.FatalLevel || entry.Level == logrus.PanicLevel {
		level = sentry.LevelFatal
	}

	brebcrumb := &sentry.Breadcrumb{
		Message: entry.Message,
		Level:   level,
	}

	if ca, ok := fields["category"].(string); ok {
		brebcrumb.Category = ca
	}

	sentry.AddBreadcrumb(brebcrumb)
	hook.client.CaptureException(errors.New(entry.Message), nil, hook.modifier)
	return nil
}
